use crate::config::AlignmentConfig;
use crate::aligner::Aligner;
use crate::alignment_mtx::{AlignmentMtx, Pointer, element};
use crate::alignment::Alignment;
use crate::alignment_mtx;

struct NtAlignmentConfig {
    subject_gap: f64,
    reference_gap: f64,
}

impl AlignmentConfig for NtAlignmentConfig {
    fn get_substitution_score(&self, pos: usize, s: char, r: char) -> f64 {
        unimplemented!()
    }
    fn get_subject_gap_opening_penalty(&self, pos: usize) -> f64 {
        self.subject_gap
    }
    fn get_reference_gap_opening_penalty(&self, pos: usize) -> f64 {
        self.reference_gap
    }
}

struct GlobalNtAligner {
    config: NtAlignmentConfig
}

impl Aligner for GlobalNtAligner {
    type Config = NtAlignmentConfig;

    fn create_mtx(&self, subject: &str, reference: &str) -> AlignmentMtx {
        alignment_mtx::of(subject.len(), reference.len())
    }

    fn fill_top_row(&self, mtx: &mut AlignmentMtx) {
        let mut gaps = 0.0;
        mtx.row_mut(0)
            .iter_mut()
            .skip(1)
            .enumerate()
            .for_each(|(i, el)| {
                gaps += self.config.get_subject_gap_opening_penalty(i);
                *el = element(gaps, Pointer::LEFT);
            });
    }

    fn fill_left_column(&self, mtx: &mut AlignmentMtx) {
        let mut gaps = 0.0;
        mtx.column_mut(0)
            .iter_mut()
            .skip(1)
            .enumerate()
            .for_each(|(i, el)| {
                gaps += self.config.get_reference_gap_opening_penalty(i);
                *el = element(gaps, Pointer::UP);
            });
    }

    fn fill(&self, mtx: &AlignmentMtx) {
        unimplemented!()
    }

    fn find_max(&self, mtx: &AlignmentMtx) -> alignment_mtx::Element {
        unimplemented!()
    }

    fn trace_back<'a>(&self, mtx: &AlignmentMtx, max: &alignment_mtx::Element) -> Alignment<'a> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::nt_aligner::{GlobalNtAligner, NtAlignmentConfig};
    use crate::aligner::Aligner;
    use crate::alignment_mtx::{Pointer};
    use crate::alignment_mtx;

    const ALIGNER: GlobalNtAligner = GlobalNtAligner {
        config: NtAlignmentConfig {
            subject_gap: 1.0,
            reference_gap: 1.0,
        }
    };

    #[test]
    fn test_create_mtx() {
        assert_eq!(
            ALIGNER.create_mtx("ss", "rrr"),
            alignment_mtx::of(2, 3)
        )
    }

    #[test]
    fn test_fill_top_row() {
        let mut mtx = alignment_mtx::of(2, 3);
        ALIGNER.fill_top_row(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            alignment_mtx::INITIAL_ELEMENT
        );
        for i in 1..3 {
            assert_eq!(
                *mtx.get((0, i)).unwrap(),
                alignment_mtx::element(i as f64, Pointer::LEFT)
            );
        }
    }

    #[test]
    fn test_fill_left_column() {
        let mut mtx = alignment_mtx::of(3, 2);
        ALIGNER.fill_left_column(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            alignment_mtx::INITIAL_ELEMENT
        );
        for i in 1..3 {
            assert_eq!(
                *mtx.get((i, 0)).unwrap(),
                alignment_mtx::element(i as f64, Pointer::UP)
            );
        }
    }

    #[test]
    fn test_match() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGCT").score,
            4.0
        )
    }

    #[test]
    fn test_mismatch() {
        assert_eq!(
            ALIGNER.align("AGAT", "AGCT").score,
            2.0
        )
    }
}