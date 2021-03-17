use crate::element::FScore;

pub trait AlignmentConfig {
    fn get_substitution_score(&self, pos: (usize, usize), s: u8, r: u8) -> FScore;
    fn get_subject_gap_opening_penalty(&self, pos: usize) -> FScore;
    fn get_reference_gap_opening_penalty(&self, pos: usize) -> FScore;
    fn get_subject_gap_extension_penalty(&self, pos: usize) -> FScore;
    fn get_reference_gap_extension_penalty(&self, pos: usize) -> FScore;
}
