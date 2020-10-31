use crate::matrix::FScore;

pub trait AlignmentConfig {
    fn get_substitution_score(&self, pos: (usize, usize), s: char, r: char) -> FScore;
    fn get_subject_gap_opening_penalty(&self, pos: usize) -> FScore;
    fn get_reference_gap_opening_penalty(&self, pos: usize) -> FScore;
}
