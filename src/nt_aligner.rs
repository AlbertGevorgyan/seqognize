use crate::config::AlignmentConfig;
use crate::aligner::{Aligner, Idx};
use crate::alignment::{Alignment, GAP};
use crate::matrix::{Matrix, Columnar, max_score, FScore};
use crate::matrix;
use crate::matrix::Element::{Deletion, Insertion, Substitution, Start};
use std::collections::VecDeque;

pub struct NtAlignmentConfig {
    pub match_score: FScore,
    pub mismatch_penalty: FScore,
    pub subject_gap_penalty: FScore,
    pub reference_gap_penalty: FScore,
}

impl AlignmentConfig for NtAlignmentConfig {
    fn get_substitution_score(&self, pos: (usize, usize), s: char, r: char) -> FScore {
        if s == r { self.match_score } else { self.mismatch_penalty }
    }
    fn get_subject_gap_opening_penalty(&self, pos: usize) -> FScore {
        self.subject_gap_penalty
    }
    fn get_reference_gap_opening_penalty(&self, pos: usize) -> FScore {
        self.reference_gap_penalty
    }
}

pub struct GlobalNtAligner {
    pub config: NtAlignmentConfig
}

impl From<NtAlignmentConfig> for GlobalNtAligner {
    fn from(config: NtAlignmentConfig) -> Self {
        GlobalNtAligner { config }
    }
}

impl Aligner<NtAlignmentConfig> for GlobalNtAligner {
    fn create_mtx(&self, subject: &str, reference: &str) -> Matrix {
        matrix::of(subject.len() + 1, reference.len() + 1)
    }

    fn fill_start(&self, mtx: &mut Matrix) {
        mtx[(0, 0)] = Start;
    }

    fn fill_top_row(&self, mtx: &mut Matrix) {
        let mut gaps = 0.0;
        mtx.row_mut(0)
            .iter_mut()
            .skip(1)
            .enumerate()
            .for_each(|(i, el)| {
                gaps += self.config.get_subject_gap_opening_penalty(i);
                *el = Deletion(gaps);
            });
    }

    fn fill_left_column(&self, mtx: &mut Matrix) {
        let mut gaps = 0.0;
        mtx.column_mut(0)
            .iter_mut()
            .skip(1)
            .enumerate()
            .for_each(|(i, el)| {
                gaps += self.config.get_reference_gap_opening_penalty(i);
                *el = Insertion(gaps);
            });
    }

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str) {
        let mut ss = subject.bytes();
        for row in 1..mtx.num_rows() {
            let s = ss.next().unwrap();
            let mut rs = reference.bytes();
            for col in 1..mtx.num_columns() {
                let r = rs.next().unwrap();
                let substitution_score = self.config.get_substitution_score((row, col), s as char, r as char);
                let insertion_penalty = self.config.get_reference_gap_opening_penalty(row);
                let deletion_penalty = self.config.get_subject_gap_opening_penalty(col);
                mtx[(row, col)] = *max_score(
                    &[
                        &Substitution(mtx[(row - 1, col - 1)].score() + substitution_score),
                        &Insertion(mtx[(row - 1, col)].score() + insertion_penalty),
                        &Deletion(mtx[(row, col - 1)].score() + deletion_penalty)
                    ]
                );
            }
        }
    }

    fn end_idx(&self, mtx: &Matrix) -> Idx {
        (mtx.num_rows() - 1, mtx.num_columns() - 1)
    }

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &str, reference: &str) -> Alignment {
        let mut ss = subject.bytes().rev();
        let mut rs = reference.bytes().rev();
        let capacity = subject.len() + reference.len();
        let mut aligned_subject = VecDeque::with_capacity(capacity);
        let mut aligned_reference = VecDeque::with_capacity(capacity);
        let mut cursor = end_index;
        while cursor != (0, 0) {
            let (row, column) = cursor;
            cursor = match mtx[cursor] {
                Substitution(_) => {
                    aligned_subject.push_front(ss.next().unwrap());
                    aligned_reference.push_front(rs.next().unwrap());
                    (row - 1, column - 1)
                }
                Insertion(_) => {
                    aligned_subject.push_front(ss.next().unwrap());
                    aligned_reference.push_front(GAP as u8);
                    (row - 1, column)
                }
                Deletion(_) => {
                    aligned_subject.push_front(GAP as u8);
                    aligned_reference.push_front(rs.next().unwrap());
                    (row, column - 1)
                }
                _ => unreachable!()
            };
        }
        Alignment::from(
            String::from_utf8(Vec::from(aligned_subject)).unwrap(),
            String::from_utf8(Vec::from(aligned_reference)).unwrap(),
            mtx[end_index].score(),
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::nt_aligner::{GlobalNtAligner, NtAlignmentConfig};
    use crate::aligner::Aligner;
    use crate::matrix::Element::{Substitution, Deletion, Insertion, Initial, Start};
    use crate::matrix;
    use crate::matrix::{FScore};
    use crate::alignment::Alignment;

    const ALIGNER: GlobalNtAligner = GlobalNtAligner {
        config: NtAlignmentConfig {
            match_score: 1.0,
            mismatch_penalty: -1.0,
            subject_gap_penalty: -1.0,
            reference_gap_penalty: -1.0,
        }
    };

    #[test]
    fn test_create_mtx() {
        assert_eq!(
            ALIGNER.create_mtx("ss", "rrr"),
            matrix::of(3, 4)
        )
    }

    #[test]
    fn test_fill_top_row() {
        let mut mtx = matrix::of(2, 3);
        ALIGNER.fill_top_row(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Initial
        );
        for i in 1..3 {
            assert_eq!(
                mtx[(0, i)],
                Deletion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_left_column() {
        let mut mtx = matrix::of(3, 2);
        ALIGNER.fill_left_column(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Initial
        );
        for i in 1..3 {
            assert_eq!(
                mtx[(i, 0)],
                Insertion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_with_match() {
        let mut mtx = matrix::from_elements(
            &[
                [Start, Deletion(-1.0)],
                [Insertion(-1.0), Substitution(0.0)]
            ]
        );
        ALIGNER.fill(&mut mtx, "A", "A");
        assert_eq!(
            mtx[(1, 1)],
            Substitution(1.0)
        );
    }

    #[test]
    fn test_trace_back_snp() {
        let mtx = matrix::from_elements(
            &[
                [Start, Deletion(-1.0)],
                [Insertion(-1.0), Substitution(1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 1), "A", "A"),
            Alignment::from("A".to_string(), "A".to_string(), 1.0)
        );
    }

    #[test]
    fn test_trace_back_insertion() {
        let mtx = matrix::from_elements(
            &[
                [Start],
                [Insertion(-1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 0), "A", ""),
            Alignment::from("A".to_string(), "_".to_string(), -1.0)
        );
    }

    #[test]
    fn test_trace_back_deletion() {
        let mtx = matrix::from_elements(
            &[
                [Start, Deletion(-1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (0, 1), "", "A"),
            Alignment::from("_".to_string(), "A".to_string(), -1.0)
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGCT"),
            Alignment::from("AGCT".to_string(), "AGCT".to_string(), 4.0)
        )
    }

    #[test]
    fn test_mismatch() {
        assert_eq!(
            ALIGNER.align("AGAT", "AGCT"),
            Alignment::from("AGAT".to_string(), "AGCT".to_string(), 2.0)
        )
    }

    #[test]
    fn test_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGT"),
            Alignment::from("AGCT".to_string(), "AG_T".to_string(), 2.0)
        )
    }

    #[test]
    fn test_deletion() {
        assert_eq!(
            ALIGNER.align("AGT", "AGCT"),
            Alignment::from("AG_T".to_string(), "AGCT".to_string(), 2.0)
        )
    }

    #[test]
    fn test_double_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AT"),
            Alignment::from("AGCT".to_string(), "A__T".to_string(), 0.0)
        )
    }

    #[test]
    fn test_double_deletion() {
        assert_eq!(
            ALIGNER.align("AT", "AGCT"),
            Alignment::from("A__T".to_string(), "AGCT".to_string(), 0.0)
        )
    }

    #[test]
    fn test_leading_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "GCT"),
            Alignment::from("AGCT".to_string(), "_GCT".to_string(), 2.0)
        )
    }

    #[test]
    fn test_leading_deletion() {
        assert_eq!(
            ALIGNER.align("GCT", "AGCT"),
            Alignment::from("_GCT".to_string(), "AGCT".to_string(), 2.0)
        )
    }

    #[test]
    fn test_trailing_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGC"),
            Alignment::from("AGCT".to_string(), "AGC_".to_string(), 2.0)
        )
    }

    #[test]
    fn test_trailing_deletion() {
        assert_eq!(
            ALIGNER.align("AGC", "AGCT"),
            Alignment::from("AGC_".to_string(), "AGCT".to_string(), 2.0)
        )
    }

    #[test]
    fn test_two_insertions() {
        assert_eq!(
            ALIGNER.align("AGCT", "GT"),
            Alignment::from("AGCT".to_string(), "_G_T".to_string(), 0.0)
        )
    }

    #[test]
    fn test_two_deletions() {
        assert_eq!(
            ALIGNER.align("AC", "AGCT"),
            Alignment::from("A_C_".to_string(), "AGCT".to_string(), 0.0)
        )
    }

    #[test]
    fn test_empty_subject() {
        assert_eq!(
            ALIGNER.align("", "AGCT"),
            Alignment::from("____".to_string(), "AGCT".to_string(), -4.0)
        )
    }

    #[test]
    fn test_empty_reference() {
        assert_eq!(
            ALIGNER.align("AGCT", ""),
            Alignment::from("AGCT".to_string(), "____".to_string(), -4.0)
        )
    }
}
