use crate::config::AlignmentConfig;
use crate::aligner::{Aligner, Idx};
use crate::alignment::Alignment;
use crate::matrix::{Matrix, Element, Columnar, max_score, FScore};
use crate::matrix;
use crate::matrix::Element::{Deletion, Insertion, Substitution, Start};

struct NtAlignmentConfig {
    match_score: FScore,
    mismatch_penalty: FScore,
    subject_gap_penalty: FScore,
    reference_gap_penalty: FScore,
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

struct GlobalNtAligner {
    config: NtAlignmentConfig
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
                *el = Insertion(gaps);
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
                *el = Deletion(gaps);
            });
    }

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str) {
        let mut ss = subject.chars();
        for row in 1..mtx.num_rows() {
            let s = ss.next().unwrap();
            let mut rs = reference.chars();
            for col in 1..mtx.num_columns() {
                let r = rs.next().unwrap();
                let substitution_score = self.config.get_substitution_score((row, col), s, r);
                let insertion_penalty = self.config.get_reference_gap_opening_penalty(row);
                let deletion_penalty = self.config.get_subject_gap_opening_penalty(col);
                mtx[(row, col)] = *max_score(
                    &[
                        &Substitution(mtx[(row - 1, col - 1)].score() + substitution_score),
                        &Insertion(mtx[(row, col - 1)].score() + insertion_penalty),
                        &Deletion(mtx[(row - 1, col)].score() + deletion_penalty)
                    ]
                );
            }
        }
    }

    fn end_idx(&self, mtx: &Matrix) -> Idx {
        (mtx.num_rows() - 1, mtx.num_columns() - 1)
    }

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &str, reference: &str) -> Alignment {
        let mut ss = subject.chars().rev();
        let mut rs = reference.chars().rev();
        let mut aligned_subject = String::new();
        let mut aligned_reference = String::new();
        let mut cursor = end_index;
        while cursor != (0, 0) {
            let (row, column) = cursor;
            cursor = match mtx[(row, column)] {
                Substitution(score) => {
                    aligned_subject.insert(0, ss.next().unwrap());
                    aligned_reference.insert(0, rs.next().unwrap());
                    (row - 1, column - 1)
                }
                Insertion(score) => {
                    aligned_subject.insert(0, ss.next().unwrap());
                    aligned_reference.insert(0, '_');
                    (row, column - 1)
                }
                Deletion(score) => {
                    aligned_subject.insert(0, '_');
                    aligned_reference.insert(0, rs.next().unwrap());
                    (row - 1, column)
                }
                _ => unreachable!()
            };
        }
        Alignment::from(aligned_subject, aligned_reference, mtx[end_index].score())
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
                Insertion(-(i as FScore))
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
                Deletion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_with_match() {
        let mut mtx = matrix::from_elements(
            &[
                [Start, Insertion(-1.0)],
                [Deletion(-1.0), Substitution(0.0)]
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
        let mut mtx = matrix::from_elements(
            &[
                [Start, Insertion(-1.0)],
                [Deletion(-1.0), Substitution(1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 1), "A", "A"),
            Alignment::from("A".to_string(), "A".to_string(), 1.0)
        );
    }

    #[test]
    fn test_trace_back_deletion() {
        let mut mtx = matrix::from_elements(
            &[
                [Start],
                [Deletion(-1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 0), "", "A"),
            Alignment::from("_".to_string(), "A".to_string(), -1.0)
        );
    }

    #[test]
    fn test_trace_back_insertion() {
        let mut mtx = matrix::from_elements(
            &[
                [Start, Insertion(-1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (0, 1), "A", "_"),
            Alignment::from("A".to_string(), "_".to_string(), -1.0)
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
}
