use crate::alignment::Alignment;
use crate::config::{AlignmentConfig};
use crate::matrix::{Matrix, Idx};

pub trait Aligner<C>: From<C>
    where C: AlignmentConfig {
    fn align(&self, subject: &str, reference: &str) -> Alignment {
        let mut mtx: Matrix = self.create_mtx(subject, reference);
        self.fill_start(&mut mtx);
        self.fill_top_row(&mut mtx);
        self.fill_left_column(&mut mtx);
        self.fill(&mut mtx, &subject, &reference);
        let end_idx: Idx = self.end_idx(&mtx);
        self.trace_back(&mtx, end_idx, &subject, &reference)
    }

    fn create_mtx(&self, subject: &str, reference: &str) -> Matrix;

    fn fill_start(&self, mtx: &mut Matrix);

    fn fill_top_row(&self, mtx: &mut Matrix);

    fn fill_left_column(&self, mtx: &mut Matrix);

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str);

    fn end_idx(&self, mtx: &Matrix) -> Idx;

    fn trace_back(&self, mtx: &Matrix, end_index: Idx, subject: &str, reference: &str) -> Alignment;
}
