use crate::alignment::Alignment;
use crate::config::{AlignmentConfig, AlignmentElement};
use crate::alignment_mtx::{AlignmentMtx, PointingScore};

pub trait Aligner<S: AlignmentElement, R: AlignmentElement> {
    type Config: AlignmentConfig<S, R>;

    fn align<'a>(&self, subject: &'a str, reference: &'a str, config: &Self::Config) -> Alignment<'a> {
        let mtx: AlignmentMtx = self.create_mtx(subject, reference);
        self.fill_top_row(&mtx, &config);
        self.fill_left_column(&mtx, &config);
        self.fill(&mtx, &config);
        let max: PointingScore = self.find_max(&mtx);
        self.trace_back(&mtx, &max)
    }

    fn create_mtx(&self, subject: &str, reference: &str) -> AlignmentMtx;

    fn fill_top_row(&self, mtx: &AlignmentMtx, config: &Self::Config);

    fn fill_left_column(&self, mtx: &AlignmentMtx, config: &Self::Config);

    fn fill(&self, mtx: &AlignmentMtx, config: &Self::Config);

    fn find_max(&self, mtx: &AlignmentMtx) -> PointingScore;

    fn trace_back<'a>(&self, mtx: &AlignmentMtx, max: &PointingScore) -> Alignment<'a>;
}

