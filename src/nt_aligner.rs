use crate::config::AlignmentConfig;
use crate::aligner::{Aligner};
use crate::alignment::{Alignment, AlignmentBuilder};
use crate::matrix::{Matrix, Idx};
use crate::{matrix};
use crate::iterators::{accumulate, set_accumulated};
use crate::element::{FScore, Element, Op, Triple};

pub struct NtAlignmentConfig {
    pub match_score: FScore,
    pub mismatch_penalty: FScore,
    pub subject_gap_opening_penalty: FScore,
    pub reference_gap_opening_penalty: FScore,
    pub subject_gap_extension_penalty: FScore,
    pub reference_gap_extension_penalty: FScore,
}

impl AlignmentConfig for NtAlignmentConfig {
    fn get_substitution_score(&self, _pos: (usize, usize), s: u8, r: u8) -> FScore {
        if s == r { self.match_score } else { self.mismatch_penalty }
    }

    fn get_subject_gap_opening_penalty(&self, _pos: usize) -> FScore {
        self.subject_gap_opening_penalty
    }

    fn get_reference_gap_opening_penalty(&self, _pos: usize) -> FScore {
        self.reference_gap_opening_penalty
    }

    fn get_subject_gap_extension_penalty(&self, _pos: usize) -> f64 {
        self.subject_gap_extension_penalty
    }

    fn get_reference_gap_extension_penalty(&self, _pos: usize) -> f64 {
        self.reference_gap_extension_penalty
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
    fn fill_top_row(&self, mtx: &mut Matrix) {
        set_accumulated(
            accumulate(
                mtx.cols(),
                |n| self.config.get_subject_gap_opening_penalty(n),
            ),
            mtx.row_mut(0).iter_mut(),
            |s| Triple::from(deletion(s), Element::inf(), Element::inf()),
        )
    }

    fn fill_left_column(&self, mtx: &mut Matrix) {
        set_accumulated(
            accumulate(
                mtx.rows(),
                |n| self.config.get_reference_gap_opening_penalty(n),
            ),
            mtx.column_mut(0).iter_mut(),
            |s| Triple::from(insertion(s), Element::inf(), Element::inf()),
        );
    }

    fn fill(&self, mtx: &mut Matrix, subject: &[u8], reference: &[u8]) {
        for row in 1..mtx.rows() {
            let s = subject[row - 1];
            for col in 1..mtx.cols() {
                let r = reference[col - 1];
                let m = select(
                    mtx[(row - 1, col - 1)].m +
                        self.config.get_substitution_score((row, col), s, r),
                    mtx[(row - 1, col)].m +
                        self.config.get_reference_gap_opening_penalty(row),
                    mtx[(row, col - 1)].m +
                        self.config.get_subject_gap_opening_penalty(col),
                );
                let x = min_score(
                    mtx[(row - 1, col)].m +
                        self.config.get_reference_gap_opening_penalty(row),
                    mtx[(row - 1, col)].x +
                        self.config.get_reference_gap_extension_penalty(row),
                );
                mtx[(row, col)] = Triple::from(m, insertion(x), mtx[(row, col)].y);
            }
        }
    }

    fn end_idx(&self, mtx: &Matrix) -> Idx {
        (mtx.rows() - 1, mtx.cols() - 1)
    }

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &[u8], reference: &[u8]) -> Alignment {
        let mut builder = AlignmentBuilder::new(subject, reference);
        let mut cursor = end_index;
        while cursor != (0, 0) {
            let element = mtx[cursor].m;
            builder.take(element.op, cursor);
            cursor = matrix::move_back(&element, cursor);
        }
        builder.take(Op::START, cursor);
        builder.build(mtx[end_index].m.score)
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

fn min_score(s1: FScore, s2: FScore) -> FScore {
    s1.min(s2)
}

pub fn insertion(score: FScore) -> Element {
    Element::from(Op::INSERT, score)
}

pub fn deletion(score: FScore) -> Element {
    Element::from(Op::DELETE, score)
}

pub fn substitution(score: FScore) -> Element {
    Element::from(Op::MATCH, score)
}

#[cfg(test)]
mod tests {
    use crate::nt_aligner::{GlobalNtAligner, NtAlignmentConfig, deletion, insertion, substitution};
    use crate::aligner::Aligner;
    use crate::matrix;
    use crate::alignment::Alignment;
    use crate::element::{FScore, Element, Triple};

    const ALIGNER: GlobalNtAligner = GlobalNtAligner {
        config: NtAlignmentConfig {
            match_score: 1.0,
            mismatch_penalty: -1.0,
            subject_gap_opening_penalty: -1.0,
            reference_gap_opening_penalty: -1.0,
            subject_gap_extension_penalty: -0.5,
            reference_gap_extension_penalty: -0.5,
        }
    };

    fn with_inf(m: Element) -> Triple {
        Triple::from(m, Element::inf(), Element::inf())
    }

    fn with_zeros(m: Element) -> Triple {
        Triple::from(m, Element::default(), Element::default())
    }

    #[test]
    fn test_fill_top_row() {
        let mut mtx = matrix::of(2, 3);
        ALIGNER.fill_top_row(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Triple::default()
        );
        for i in 1..3 {
            assert_eq!(
                mtx[(0, i)],
                with_inf(deletion(-(i as FScore)))
            );
        }
    }

    #[test]
    fn test_fill_left_column() {
        let mut mtx = matrix::of(3, 2);
        ALIGNER.fill_left_column(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Triple::default()
        );
        for i in 1..3 {
            assert_eq!(
                mtx[(i, 0)],
                with_inf(insertion(-(i as FScore)))
            );
        }
    }

    #[test]
    fn test_fill_with_match() {
        let mut mtx = matrix::from_elements(
            &[
                [Triple::default(), with_inf(deletion(-1.0))],
                [with_inf(insertion(-1.0)), with_zeros(substitution(0.0))]
            ]
        );
        ALIGNER.fill(&mut mtx, "A".as_bytes(), "A".as_bytes());
        assert_eq!(
            mtx[(1, 1)],
            with_zeros(substitution(1.0))
        );
    }

    #[test]
    fn test_trace_back_snp() {
        let mtx = matrix::from_elements(
            &[
                [Triple::default(), with_inf(deletion(-1.0))],
                [with_inf(insertion(-1.0)), with_zeros(substitution(1.0))]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 1), "A".as_bytes(), "A".as_bytes()),
            Alignment::from("A", "A", 1.0)
        );
    }

    #[test]
    fn test_trace_back_insertion() {
        let mtx = matrix::from_elements(
            &[
                [Triple::default()],
                [with_inf(insertion(-1.0))]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (1, 0), &['A' as u8], &[]),
            Alignment::from("A", "_", -1.0)
        );
    }

    #[test]
    fn test_trace_back_deletion() {
        let mtx = matrix::from_elements(
            &[
                [Triple::default(), with_inf(deletion(-1.0))]
            ]
        );
        assert_eq!(
            ALIGNER.trace_back(&mtx, (0, 1), &[], &['A' as u8]),
            Alignment::from("_", "A", -1.0)
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b"AGCT"),
            Alignment::from("AGCT", "AGCT", 4.0)
        )
    }

    #[test]
    fn test_mismatch() {
        assert_eq!(
            ALIGNER.align(b"AGAT", b"AGCT"),
            Alignment::from("AGAT", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_insertion() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b"AGT"),
            Alignment::from("AGCT", "AG_T", 2.0)
        )
    }

    #[test]
    fn test_deletion() {
        assert_eq!(
            ALIGNER.align(b"AGT", b"AGCT"),
            Alignment::from("AG_T", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_double_insertion() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b"AT"),
            Alignment::from("AGCT", "A__T", 0.0)
        )
    }

    #[test]
    fn test_double_deletion() {
        assert_eq!(
            ALIGNER.align(b"AT", b"AGCT"),
            Alignment::from("A__T", "AGCT", 0.0)
        )
    }

    #[test]
    fn test_leading_insertion() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b"GCT"),
            Alignment::from("AGCT", "_GCT", 2.0)
        )
    }

    #[test]
    fn test_leading_deletion() {
        assert_eq!(
            ALIGNER.align(b"GCT", b"AGCT"),
            Alignment::from("_GCT", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_trailing_insertion() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b"AGC"),
            Alignment::from("AGCT", "AGC_", 2.0)
        )
    }

    #[test]
    fn test_trailing_deletion() {
        assert_eq!(
            ALIGNER.align(b"AGC", b"AGCT"),
            Alignment::from("AGC_", "AGCT", 2.0)
        )
    }

    #[test]
    fn test_two_insertions() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b"GT"),
            Alignment::from("AGCT", "_G_T", 0.0)
        )
    }

    #[test]
    fn test_two_deletions() {
        assert_eq!(
            ALIGNER.align(b"AC", b"AGCT"),
            Alignment::from("A_C_", "AGCT", 0.0)
        )
    }

    #[test]
    fn test_empty_subject() {
        assert_eq!(
            ALIGNER.align(b"", b"AGCT"),
            Alignment::from("____", "AGCT", -4.0)
        )
    }

    #[test]
    fn test_empty_reference() {
        assert_eq!(
            ALIGNER.align(b"AGCT", b""),
            Alignment::from("AGCT", "____", -4.0)
        )
    }

    #[test]
    #[ignore]
    fn test_affine_gap() {
        assert_eq!(
            ALIGNER.align("A", "ACG"),
            Alignment::from("A__", "ACG", -0.5)
        )
    }
}

