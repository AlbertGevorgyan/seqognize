use crate::alignment::Alignment;
use crate::config::{AlignmentConfig};
use crate::matrix::{Matrix, Idx};
use crate::matrix;

pub trait Aligner<C>: From<C>
    where C: AlignmentConfig {

    fn align(&self, subject: &[u8], reference: &[u8]) -> Alignment {
        let mut mtx = matrix::of(subject.len() + 1, reference.len() + 1);
        self.fill_top_row(&mut mtx);
        self.fill_left_column(&mut mtx);
        self.fill(&mut mtx, subject, reference);
        let end_idx: Idx = self.end_idx(&mtx);
        self.trace_back(&mtx, end_idx, &subject, &reference)
    }

    fn fill_top_row(&self, mtx: &mut Matrix);

    fn fill_left_column(&self, mtx: &mut Matrix);

    fn fill(&self, mtx: &mut Matrix, subject: &[u8], reference: &[u8]);

    fn end_idx(&self, mtx: &Matrix) -> Idx;

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &[u8], reference: &[u8]) -> Alignment;
}
