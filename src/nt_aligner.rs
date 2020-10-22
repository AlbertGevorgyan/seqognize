use crate::config::AlignmentConfig;
use crate::aligner::Aligner;
use crate::alignment_mtx::{AlignmentMtx};
use crate::alignment::Alignment;
use crate::alignment_mtx;

struct GlobalNtAligner {}

struct NtAlignmentConfig {}

impl AlignmentConfig<char, char> for NtAlignmentConfig {
    fn get_substitution_score(&self, pos: usize, s: char, r: char) -> f64 {
        unimplemented!()
    }

    fn get_subject_gap_opening_penalty(&self, pos: usize) -> f64 {
        unimplemented!()
    }

    fn get_reference_gap_opening_penalty(&self, pos: usize) -> f64 {
        unimplemented!()
    }
}

impl Aligner<char, char> for GlobalNtAligner {
    type Config = NtAlignmentConfig;

    fn create_mtx(&self, subject: &str, reference: &str) -> AlignmentMtx {
        alignment_mtx::of(subject.len(), reference.len())
    }

    fn fill_top_row(&self, mtx: &AlignmentMtx, config: &Self::Config) {
        let mut gaps = 0.0;
        for (i, el) in mtx.row_iter(0).skip(1).enumerate() {
            gaps += config.get_subject_gap_opening_penalty(i);
        }
    }

    fn fill_left_column(&self, mtx: &AlignmentMtx, config: &Self::Config) {
        unimplemented!()
    }

    fn fill(&self, mtx: &AlignmentMtx, config: &Self::Config) {
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

    #[test]
    fn test_create_mtx() {
        let aligner = GlobalNtAligner {};
        assert_eq!(
            aligner.create_mtx("ss", "rrr"),
            alignment_mtx::of(2, 3)
        )
    }

    #[test]
    fn test_fill_top_row() {
        let aligner = GlobalNtAligner {};
        let mtx = alignment_mtx::of(2, 3);
        let config = NtAlignmentConfig {};
        aligner.fill_top_row(&mtx, &config);
        assert_eq!(
            *mtx.get(0, 0).unwrap(),
            alignment_mtx::INITIAL_ELEMENT
        );
        assert_eq!(
            *mtx.get(0, 1).unwrap(),
            alignment_mtx::element(1.0, Pointer::LEFT)
        );
    }

    #[test]
    fn test_match() {
        let aligner = GlobalNtAligner {};
        let config = NtAlignmentConfig {};
        assert_eq!(
            aligner.align("AGCT", "AGCT", &config).score,
            4.0
        )
    }

    #[test]
    fn test_mismatch() {
        let aligner = GlobalNtAligner {};
        let config = NtAlignmentConfig {};
        assert_eq!(
            aligner.align("AGAT", "AGCT", &config).score,
            2.0
        )
    }
}