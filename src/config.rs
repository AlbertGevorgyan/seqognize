pub trait AlignmentConfig {
    fn get_substitution_score(&self, pos: (usize, usize), s: char, r: char) -> f64;
    fn get_subject_gap_opening_penalty(&self, pos: usize) -> f64;
    fn get_reference_gap_opening_penalty(&self, pos: usize) -> f64;
}
