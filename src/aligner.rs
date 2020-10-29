use crate::alignment::Alignment;
use crate::config::{AlignmentConfig};
use crate::alignment_mtx::{AlignmentMtx, Element};
use crate::alignment_mtx;

pub trait Aligner {
    type Config: AlignmentConfig;

    fn align<'a>(&self, subject: &'a str, reference: &'a str) -> Alignment<'a> {
        let mut mtx: AlignmentMtx = self.create_mtx(subject, reference);
        self.fill_top_row(&mut mtx);
        self.fill_left_column(&mut mtx);
        self.fill(&mtx);
        let max: Element = self.find_max(&mtx);
        self.trace_back(&mtx, &max)
    }

    fn create_mtx(&self, subject: &str, reference: &str) -> AlignmentMtx;

    fn fill_top_row(&self, mtx: &mut AlignmentMtx);

    fn fill_left_column(&self, mtx: &mut AlignmentMtx);

    fn fill(&self, mtx: &AlignmentMtx);

    fn find_max(&self, mtx: &AlignmentMtx) -> alignment_mtx::Element;

    fn trace_back<'a>(&self, mtx: &AlignmentMtx, max: &alignment_mtx::Element) -> Alignment<'a>;
}

