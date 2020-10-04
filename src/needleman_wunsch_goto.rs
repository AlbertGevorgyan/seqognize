use crate::alignment_config::AlignmentConfig;
use crate::dynamic_program::DynamicProgram;
use crate::alignment_mtx::{AlignmentMtx, PointingScore};
use crate::alignment::Alignment;

struct NeedlemanWunschGotoProgram {}

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

impl DynamicProgram<char, char> for NeedlemanWunschGotoProgram {
    type Config = NtAlignmentConfig;

    fn create_mtx(&self, subject: &str, reference: &str) -> AlignmentMtx {
        unimplemented!()
    }

    fn fill_top_row(&self, mtx: &AlignmentMtx, config: &Self::Config) {
        unimplemented!()
    }

    fn fill_left_column(&self, mtx: &AlignmentMtx, config: &Self::Config) {
        unimplemented!()
    }

    fn fill(&self, mtx: &AlignmentMtx, config: &Self::Config) {
        unimplemented!()
    }

    fn find_max(&self, mtx: &AlignmentMtx) -> PointingScore {
        unimplemented!()
    }

    fn trace_back<'a>(&self, mtx: &AlignmentMtx, max: &PointingScore) -> Alignment<'a> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::alignment::Alignment;
    use crate::alignment_config::tests::TestConfig;
    use crate::needleman_wunsch_goto::{NeedlemanWunschGotoProgram, NtAlignmentConfig};
    use crate::dynamic_program::DynamicProgram;

    #[test]
    fn test_align() {
        let p = NeedlemanWunschGotoProgram {};
        let c = NtAlignmentConfig {};
        assert_eq!(
            p.align("AGCT", "AGCT", &c).score,
            4.0
        )
    }
}