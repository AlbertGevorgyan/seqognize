use crate::config::AlignmentConfig;
use crate::aligner::{Aligner};
use crate::alignment::{Alignment, AlignmentBuilder};
use crate::matrix::{Matrix, Idx};
use crate::{matrix};
use crate::iterators::{SeqIterator, accumulate, set_accumulated};
use crate::element::{FScore, Element, Pointer};

pub struct NtAlignmentConfig {
    pub match_score: FScore,
    pub mismatch_penalty: FScore,
    pub subject_gap_penalty: FScore,
    pub reference_gap_penalty: FScore,
}

impl AlignmentConfig for NtAlignmentConfig {
    fn get_substitution_score(&self, pos: (usize, usize), s: u8, r: u8) -> FScore {
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

    fn fill_top_row(&self, mtx: &mut Matrix) {
        set_accumulated(
            accumulate(
                mtx.cols(),
                |n| self.config.get_subject_gap_opening_penalty(n),
            ),
            mtx.row_mut(0).iter_mut(),
            |s| deletion(s),
        )
    }

    fn fill_left_column(&self, mtx: &mut Matrix) {
        set_accumulated(
            accumulate(
                mtx.rows(),
                |n| self.config.get_reference_gap_opening_penalty(n),
            ),
            mtx.column_mut(0).iter_mut(),
            |s| insertion(s),
        );
    }

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str) {
        let mut subject_iterator = SeqIterator::from(subject);
        for row in 1..mtx.rows() {
            let s = subject_iterator.next_byte();
            let mut reference_iterator = SeqIterator::from(reference);
            for col in 1..mtx.cols() {
                let r = reference_iterator.next_byte();
                mtx[(row, col)] = select(
                    mtx[(row - 1, col - 1)] +
                        self.config.get_substitution_score((row, col), s, r),
                    mtx[(row - 1, col)] +
                        self.config.get_reference_gap_opening_penalty(row),
                    mtx[(row, col - 1)] +
                        self.config.get_subject_gap_opening_penalty(col),
                )
            }
        }
    }

    fn end_idx(&self, mtx: &Matrix) -> Idx {
        (mtx.rows() - 1, mtx.cols() - 1)
    }

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &str, reference: &str) -> Alignment {
        let mut builder = AlignmentBuilder::new(subject, reference);
        let mut cursor = end_index;
        while cursor != (0, 0) {
            let element = mtx[cursor];
            builder.take(element.pointer, cursor);
            cursor = matrix::move_back(&element, cursor);
        }
        builder.build(mtx[end_index].score)
    }
}

fn select(substitution_score: FScore, insertion_score: FScore, deletion_score: FScore) -> Element {
    if substitution_score >= insertion_score && substitution_score >= deletion_score {
        substitution(substitution_score)
    } else if insertion_score >= deletion_score {
        insertion(insertion_score)
    } else {
        deletion(deletion_score)
    }
}

pub fn insertion(score: FScore) -> Element {
    Element { pointer: Pointer::UP, score }
}

pub fn deletion(score: FScore) -> Element {
    Element { pointer: Pointer::LEFT, score }
}

pub fn substitution(score: FScore) -> Element {
    Element { pointer: Pointer::DIAGONAL, score }
}

#[cfg(test)]
mod tests {
    use crate::nt_aligner::{GlobalNtAligner, NtAlignmentConfig, deletion, insertion, substitution};
    use crate::aligner::Aligner;
    use crate::matrix;
    use crate::matrix::{Matrix};
    use crate::alignment::Alignment;
    use crate::element::{FScore, Element};

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
            Element::default()
        );
        for i in 1..3 {
            assert_eq!(
                mtx[(0, i)],
                deletion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_left_column() {
        let mut mtx = matrix::of(3, 2);
        ALIGNER.fill_left_column(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Element::default()
        );
        for i in 1..3 {
            assert_eq!(
                mtx[(i, 0)],
                insertion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_with_match() {
        let mut mtx = matrix::from_elements(
            &[
                [Element::default(), deletion(-1.0)],
                [insertion(-1.0), substitution(0.0)]
            ]
        );
        ALIGNER.fill(&mut mtx, "A", "A");
        assert_eq!(
            mtx[(1, 1)],
            substitution(1.0)
        );
    }

    #[test]
    fn test_trace_back_snp() {
        let mtx = matrix::from_elements(
            &[
                [Element::default(), deletion(-1.0)],
                [insertion(-1.0), substitution(1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 1), "A", "A"),
            Alignment::from("A", "A", 1.0)
        );
    }

    #[test]
    fn test_trace_back_insertion() {
        let mtx = matrix::from_elements(
            &[
                [Element::default()],
                [insertion(-1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 0), "A", ""),
            Alignment::from("A", "_", -1.0)
        );
    }

    #[test]
    fn test_trace_back_deletion() {
        let mtx = matrix::from_elements(
            &[
                [Element::default(), deletion(-1.0)]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (0, 1), "", "A"),
            Alignment::from("_", "A", -1.0)
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGCT"),
            Alignment::from("AGCT", "AGCT", 4.0)
        )
    }

    #[test]
    fn test_mismatch() {
        assert_eq!(
            ALIGNER.align("AGAT", "AGCT"),
            Alignment::from("AGAT", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGT"),
            Alignment::from("AGCT", "AG_T", 2.0)
        )
    }

    #[test]
    fn test_deletion() {
        assert_eq!(
            ALIGNER.align("AGT", "AGCT"),
            Alignment::from("AG_T", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_double_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AT"),
            Alignment::from("AGCT", "A__T", 0.0)
        )
    }

    #[test]
    fn test_double_deletion() {
        assert_eq!(
            ALIGNER.align("AT", "AGCT"),
            Alignment::from("A__T", "AGCT", 0.0)
        )
    }

    #[test]
    fn test_leading_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "GCT"),
            Alignment::from("AGCT", "_GCT", 2.0)
        )
    }

    #[test]
    fn test_leading_deletion() {
        assert_eq!(
            ALIGNER.align("GCT", "AGCT"),
            Alignment::from("_GCT", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_trailing_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGC"),
            Alignment::from("AGCT", "AGC_", 2.0)
        )
    }

    #[test]
    fn test_trailing_deletion() {
        assert_eq!(
            ALIGNER.align("AGC", "AGCT"),
            Alignment::from("AGC_", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_two_insertions() {
        assert_eq!(
            ALIGNER.align("AGCT", "GT"),
            Alignment::from("AGCT", "_G_T", 0.0)
        )
    }

    #[test]
    fn test_two_deletions() {
        assert_eq!(
            ALIGNER.align("AC", "AGCT"),
            Alignment::from("A_C_", "AGCT", 0.0)
        )
    }

    #[test]
    fn test_empty_subject() {
        assert_eq!(
            ALIGNER.align("", "AGCT"),
            Alignment::from("____", "AGCT", -4.0)
        )
    }

    #[test]
    fn test_empty_reference() {
        assert_eq!(
            ALIGNER.align("AGCT", ""),
            Alignment::from("AGCT", "____", -4.0)
        )
    }
}

