use crate::alignment::Alignment;
use crate::config::{AlignmentConfig};
use crate::matrix::{Matrix, Element};

pub trait Aligner<C>: From<C>
    where C: AlignmentConfig {
    fn align<'a>(&self, subject: &'a str, reference: &'a str) -> Alignment<'a> {
        let mut mtx: Matrix = self.create_mtx(subject, reference);
        self.fill_top_row(&mut mtx);
        self.fill_left_column(&mut mtx);
        self.fill(&mut mtx, &subject, &reference);
        let max: Element = self.find_max(&mtx);
        self.trace_back(&mtx, &max)
    }

    fn create_mtx(&self, subject: &str, reference: &str) -> Matrix;

    fn fill_top_row(&self, mtx: &mut Matrix);

    fn fill_left_column(&self, mtx: &mut Matrix);

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str);

    fn find_max(&self, mtx: &Matrix) -> Element;

    fn trace_back<'a>(&self, mtx: &Matrix, max: &Element) -> Alignment<'a>;
}
