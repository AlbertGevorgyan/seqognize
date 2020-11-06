use crate::config::AlignmentConfig;
use crate::aligner::{Aligner};
use crate::alignment::{Alignment, AlignmentBuilder};
use crate::matrix::{Matrix, Columnar, FScore, Element, Idx};
use crate::{matrix};
use crate::matrix::Element::{Deletion, Insertion, Substitution, Start};
use crate::iterators::{SeqIterator, accumulate};
use ndarray::{ArrayBase, ViewRepr, Dimension};

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
        Matrix::of(subject.len() + 1, reference.len() + 1)
    }

    fn fill_start(&self, mtx: &mut Matrix) {
        mtx[(0, 0)] = Start;
    }

    fn fill_top_row(&self, mtx: &mut Matrix) {
        let accumulator = accumulate(
            mtx.num_columns(),
            |n| self.config.get_subject_gap_opening_penalty(n),
        );
        fill_gaps(
            &mut mtx.row_mut(0),
            accumulator,
            |s| Deletion(s),
        )
    }

    fn fill_left_column(&self, mtx: &mut Matrix) {
        let accumulator = accumulate(
            mtx.num_rows(),
            |n| self.config.get_reference_gap_opening_penalty(n),
        );
        fill_gaps(
            &mut mtx.column_mut(0),
            accumulator,
            |s| Insertion(s),
        );
    }

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str) {
        let mut subject_iterator = SeqIterator::from(subject);
        for row in 1..mtx.num_rows() {
            let s = subject_iterator.next_byte();
            let mut reference_iterator = SeqIterator::from(reference);
            for col in 1..mtx.num_columns() {
                let r = reference_iterator.next_byte();
                mtx[(row, col)] = select(
                    mtx[(row - 1, col - 1)].score() +
                        self.config.get_substitution_score((row, col), s, r),
                    mtx[(row - 1, col)].score() +
                        self.config.get_reference_gap_opening_penalty(row),
                    mtx[(row, col - 1)].score() +
                        self.config.get_subject_gap_opening_penalty(col),
                )
            }
        }
    }

    fn end_idx(&self, mtx: &Matrix) -> Idx {
        (mtx.num_rows() - 1, mtx.num_columns() - 1)
    }

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &str, reference: &str) -> Alignment {
        let mut builder = AlignmentBuilder::new(subject, reference);
        let mut cursor = end_index;
        while cursor != (0, 0) {
            let element = mtx[cursor];
            builder.take(&element);
            cursor = matrix::move_back(&element, cursor);
        }
        builder.build(mtx[end_index].score())
    }
}

fn fill_gaps(
    dimension: &mut ArrayBase<ViewRepr<&mut Element>, impl Dimension>,
    accumulator: impl Iterator<Item=FScore>,
    element: fn(FScore) -> Element,
) {
    dimension.iter_mut()
        .skip(1)
        .zip(accumulator.skip(1))
        .for_each(|(el, gaps)| *el = element(gaps));
}

fn select(substitution_score: FScore, insertion_score: FScore, deletion_score: FScore) -> Element {
    if substitution_score >= insertion_score && substitution_score >= deletion_score {
        Substitution(substitution_score)
    } else if insertion_score >= deletion_score {
        Insertion(insertion_score)
    } else {
        Deletion(deletion_score)
    }
}

#[cfg(test)]
mod tests {
    use crate::nt_aligner::{GlobalNtAligner, NtAlignmentConfig};
    use crate::aligner::Aligner;
    use crate::matrix::Element::{Substitution, Deletion, Insertion, Initial, Start};
    use crate::matrix;
    use crate::matrix::{Columnar, Matrix, FScore};
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
            Matrix::of(3, 4)
        )
    }

    #[test]
    fn test_fill_top_row() {
        let mut mtx = Matrix::of(2, 3);
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
        let mut mtx = Matrix::of(3, 2);
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
            Alignment::new("A", "A", 1.0)
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
            Alignment::new("A", "_", -1.0)
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
            Alignment::new("_", "A", -1.0)
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGCT"),
            Alignment::new("AGCT", "AGCT", 4.0)
        )
    }

    #[test]
    fn test_mismatch() {
        assert_eq!(
            ALIGNER.align("AGAT", "AGCT"),
            Alignment::new("AGAT", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGT"),
            Alignment::new("AGCT", "AG_T", 2.0)
        )
    }

    #[test]
    fn test_deletion() {
        assert_eq!(
            ALIGNER.align("AGT", "AGCT"),
            Alignment::new("AG_T", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_double_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AT"),
            Alignment::new("AGCT", "A__T", 0.0)
        )
    }

    #[test]
    fn test_double_deletion() {
        assert_eq!(
            ALIGNER.align("AT", "AGCT"),
            Alignment::new("A__T", "AGCT", 0.0)
        )
    }

    #[test]
    fn test_leading_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "GCT"),
            Alignment::new("AGCT", "_GCT", 2.0)
        )
    }

    #[test]
    fn test_leading_deletion() {
        assert_eq!(
            ALIGNER.align("GCT", "AGCT"),
            Alignment::new("_GCT", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_trailing_insertion() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGC"),
            Alignment::new("AGCT", "AGC_", 2.0)
        )
    }

    #[test]
    fn test_trailing_deletion() {
        assert_eq!(
            ALIGNER.align("AGC", "AGCT"),
            Alignment::new("AGC_", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_two_insertions() {
        assert_eq!(
            ALIGNER.align("AGCT", "GT"),
            Alignment::new("AGCT", "_G_T", 0.0)
        )
    }

    #[test]
    fn test_two_deletions() {
        assert_eq!(
            ALIGNER.align("AC", "AGCT"),
            Alignment::new("A_C_", "AGCT", 0.0)
        )
    }

    #[test]
    fn test_empty_subject() {
        assert_eq!(
            ALIGNER.align("", "AGCT"),
            Alignment::new("____", "AGCT", -4.0)
        )
    }

    #[test]
    fn test_empty_reference() {
        assert_eq!(
            ALIGNER.align("AGCT", ""),
            Alignment::new("AGCT", "____", -4.0)
        )
    }
}

