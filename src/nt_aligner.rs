use crate::config::AlignmentConfig;
use crate::aligner::Aligner;
use crate::alignment::Alignment;
use crate::matrix::{Matrix, Element, Columnar, max_score, FScore};
use crate::matrix;
use crate::matrix::Element::{Deletion, Insertion, Substitution};

struct NtAlignmentConfig {
    match_score: FScore,
    mismatch_penalty: FScore,
    subject_gap_penalty: FScore,
    reference_gap_penalty: FScore,
}

impl AlignmentConfig for NtAlignmentConfig {
    fn get_substitution_score(&self, pos: (usize, usize), s: char, r: char) -> FScore {
        if s == r { self.match_score } else { self.mismatch_penalty }
    }
    fn get_subject_gap_opening_penalty(&self, pos: usize) -> FScore {
        self.subject_gap_penalty
    }
    fn get_reference_gap_opening_penalty(&self, pos: usize) -> FScore {
        self.reference_gap_penalty
    }
}

struct GlobalNtAligner {
    config: NtAlignmentConfig
}

impl From<NtAlignmentConfig> for GlobalNtAligner {
    fn from(config: NtAlignmentConfig) -> Self {
        GlobalNtAligner { config }
    }
}

impl Aligner<NtAlignmentConfig> for GlobalNtAligner {
    fn create_mtx(&self, subject: &str, reference: &str) -> Matrix {
        matrix::of(subject.len(), reference.len())
    }

    fn fill_top_row(&self, mtx: &mut Matrix) {
        let mut gaps = 0.0;
        mtx.row_mut(0)
            .iter_mut()
            .skip(1)
            .enumerate()
            .for_each(|(i, el)| {
                gaps += self.config.get_subject_gap_opening_penalty(i);
                *el = Insertion(gaps);
            });
    }

    fn fill_left_column(&self, mtx: &mut Matrix) {
        let mut gaps = 0.0;
        mtx.column_mut(0)
            .iter_mut()
            .skip(1)
            .enumerate()
            .for_each(|(i, el)| {
                gaps += self.config.get_reference_gap_opening_penalty(i);
                *el = Deletion(gaps);
            });
    }

    fn fill(&self, mtx: &mut Matrix, subject: &str, reference: &str) {
        let mut ss = subject.chars();
        let mut rs = reference.chars();
        for row in 1..mtx.num_rows() {
            for col in 1..mtx.num_columns() {
                let substitution_score = self.config.get_substitution_score(
                    (row, col),
                    ss.next().unwrap(),
                    rs.next().unwrap(),
                );
                let insertion_penalty = self.config.get_reference_gap_opening_penalty(row);
                let deletion_penalty = self.config.get_subject_gap_opening_penalty(col);
                mtx[(row, col)] = *max_score(
                    &[
                        &Substitution(mtx[(row - 1, col - 1)].score() + substitution_score),
                        &Insertion(mtx[(row, col - 1)].score() + insertion_penalty),
                        &Deletion(mtx[(row - 1, col)].score() + deletion_penalty)
                    ]
                );
            }
        }
    }

    fn find_max(&self, mtx: &Matrix) -> Element {
        unimplemented!()
    }

    fn trace_back<'a>(&self, mtx: &Matrix, max: &Element) -> Alignment<'a> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use crate::nt_aligner::{GlobalNtAligner, NtAlignmentConfig};
    use crate::aligner::Aligner;
    use crate::matrix::Element::{Substitution, Deletion, Insertion, Initial, Start};
    use crate::matrix;
    use crate::matrix::{FScore};

    const ALIGNER: GlobalNtAligner = GlobalNtAligner {
        config: NtAlignmentConfig {
            match_score: 1.0,
            mismatch_penalty: -1.0,
            subject_gap_penalty: -1.0,
            reference_gap_penalty: -1.0,
        }
    };

    #[test]
    fn test_create_mtx() {
        assert_eq!(
            ALIGNER.create_mtx("ss", "rrr"),
            matrix::of(2, 3)
        )
    }

    #[test]
    fn test_fill_top_row() {
        let mut mtx = matrix::of(2, 3);
        ALIGNER.fill_top_row(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Initial
        );
        for i in 1..3 {
            assert_eq!(
                *mtx.get((0, i)).unwrap(),
                Insertion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_left_column() {
        let mut mtx = matrix::of(3, 2);
        ALIGNER.fill_left_column(&mut mtx);
        assert_eq!(
            *mtx.get((0, 0)).unwrap(),
            Initial
        );
        for i in 1..3 {
            assert_eq!(
                *mtx.get((i, 0)).unwrap(),
                Deletion(-(i as FScore))
            );
        }
    }

    #[test]
    fn test_fill_with_match() {
        let mut mtx = matrix::from_elements(
            &[
                [
                    Start,
                    Insertion(-1.0)
                ],
                [
                    Deletion(-1.0),
                    Substitution(0.0)
                ]
            ]
        );
        ALIGNER.fill(&mut mtx, "A", "A");
        assert_eq!(
            *mtx.get((1, 1)).unwrap(),
            Substitution(1.0)
        );
    }

    #[test]
    fn test_fill_with_mismatch() {
        let mut mtx = matrix::from_elements(
            &[
                [
                    Start,
                    Insertion(-1.0)
                ],
                [
                    Deletion(-1.0),
                    Substitution(0.0)
                ]
            ]
        );
        ALIGNER.fill(&mut mtx, "A", "G");
        assert_eq!(
            *mtx.get((1, 1)).unwrap(),
            Substitution(-1.0)
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            ALIGNER.align("AGCT", "AGCT").score,
            4.0
        )
    }

    #[test]
    fn test_mismatch() {
        assert_eq!(
            ALIGNER.align("AGAT", "AGCT").score,
            2.0
        )
    }
}
