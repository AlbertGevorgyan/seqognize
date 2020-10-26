use crate::config::AlignmentConfig;
use crate::aligner::Aligner;
use crate::alignment_mtx::{AlignmentMtx, Pointer, ScoreMtx};
use crate::alignment::Alignment;
use crate::alignment_mtx;

struct GlobalNtAligner {}

struct NtAlignmentConfig {
    subject_gap: f64,
    reference_gap: f64,
}

impl AlignmentConfig<char, char> for NtAlignmentConfig {
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

impl Aligner<char, char> for GlobalNtAligner {
    type Config = NtAlignmentConfig;

    fn create_mtx(&self, subject: &str, reference: &str) -> AlignmentMtx {
        alignment_mtx::of(subject.len(), reference.len())
    }

    fn fill_top_row(&self, mtx: &mut AlignmentMtx, config: &Self::Config) {
        let mut gaps = 0.0;
        for i in 1..mtx.num_columns() {
            gaps += config.get_subject_gap_opening_penalty(i);
            mtx.put(0, i, gaps, Pointer::LEFT);
        }
    }

    fn fill_left_column(&self, mtx: &mut AlignmentMtx, config: &Self::Config) {
        let mut gaps = 0.0;
        for i in 1..mtx.num_rows() {
            gaps += config.get_reference_gap_extension_penalty(i);
            mtx.put(i, 0, gaps, Pointer::UP);
        }
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
        let mut mtx = alignment_mtx::of(2, 3);
        let config = NtAlignmentConfig {
            subject_gap: 1.0,
            reference_gap: 1.0,
        };
        aligner.fill_top_row(&mut mtx, &config);
        assert_eq!(
            *mtx.get(0, 0).unwrap(),
            alignment_mtx::INITIAL_ELEMENT
        );
        for i in 1..3 {
            assert_eq!(
                *mtx.get(0, i).unwrap(),
                alignment_mtx::element(i as f64, Pointer::LEFT)
            );
        }
    }

    #[test]
    fn test_fill_left_column() {
        let aligner = GlobalNtAligner {};
        let mut mtx = alignment_mtx::of(3, 2);
        let config = NtAlignmentConfig {
            subject_gap: 1.0,
            reference_gap: 1.0,
        };
        aligner.fill_left_column(&mut mtx, &config);
        assert_eq!(
            *mtx.get(0, 0).unwrap(),
            alignment_mtx::INITIAL_ELEMENT
        );
        for i in 1..3 {
            assert_eq!(
                *mtx.get(i, 0).unwrap(),
                alignment_mtx::element(i as f64, Pointer::UP)
            );
        }
    }

    #[test]
    fn test_match() {
        let aligner = GlobalNtAligner {};
        let config = NtAlignmentConfig {
            subject_gap: 1.0,
            reference_gap: 1.0,
        };
        assert_eq!(
            aligner.align("AGCT", "AGCT", &config).score,
            4.0
        )
    }

    #[test]
    fn test_mismatch() {
        let aligner = GlobalNtAligner {};
        let config = NtAlignmentConfig {
            subject_gap: 1.0,
            reference_gap: 1.0,
        };
        assert_eq!(
            aligner.align("AGAT", "AGCT", &config).score,
            2.0
        )
    }
}