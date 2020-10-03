use std::str::FromStr;

trait AlignmentElement: FromStr {}

impl AlignmentElement for char {}

trait AlignmentConfig<S: AlignmentElement, R: AlignmentElement> {
    fn get_substitution_score(&self, pos: usize, s: S, r: R) -> f64;

    fn get_subject_gap_opening_penalty(&self, pos: usize) -> f64;

    fn get_reference_gap_opening_penalty(&self, pos: usize) -> f64;

    fn get_subject_gap_extension_penalty(&self, pos: usize) -> f64 {
        self.get_subject_gap_opening_penalty(pos)
    }

    fn get_reference_gap_extension_penalty(&self, pos: usize) -> f64 {
        self.get_reference_gap_opening_penalty(pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::alignment_config::AlignmentConfig;

    struct TestConfig {}

    impl AlignmentConfig<char, char> for TestConfig {
        fn get_substitution_score(&self, pos: usize, s: char, r: char) -> f64 {
            if s == r { 1.0 } else { -1.0 }
        }

        fn get_subject_gap_opening_penalty(&self, pos: usize) -> f64 {
            -1.0
        }

        fn get_reference_gap_opening_penalty(&self, pos: usize) -> f64 {
            -1.0
        }
    }

    const CONFIG: TestConfig = TestConfig {};

    #[test]
    fn test_get_substitution_score() {
        assert_eq!(
            CONFIG.get_substitution_score(0, 'a', 'a'),
            1.0
        );
        assert_eq!(
            CONFIG.get_substitution_score(0, 'a', 'b'),
            -1.0
        );
    }

    #[test]
    fn test_get_subject_gap_penalty() {
        assert_eq!(
            CONFIG.get_subject_gap_opening_penalty(0),
            CONFIG.get_reference_gap_extension_penalty(0)
        )
    }

    #[test]
    fn test_get_reference_gap_penalty() {
        assert_eq!(
            CONFIG.get_reference_gap_opening_penalty(0),
            CONFIG.get_reference_gap_extension_penalty(0)
        )
    }
}