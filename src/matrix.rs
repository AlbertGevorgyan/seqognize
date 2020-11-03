use ndarray::{Array2, FixedInitializer, arr2};
use crate::matrix::Element::{Start, Insertion, Deletion, Substitution, Initial};

pub type FScore = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Element {
    Initial,
    Start,
    Insertion(FScore),
    Deletion(FScore),
    Substitution(FScore),
}

impl Element {
    pub fn score(&self) -> FScore {
        return match self {
            Start => 0.0,
            Insertion(score) | Deletion(score) | Substitution(score) => *score,
            Initial => unreachable!()
        };
    }
}

pub trait Columnar {
    fn num_rows(&self) -> usize;
    fn num_columns(&self) -> usize;
}

pub type Matrix = Array2<Element>;

impl Columnar for Matrix {
    fn num_rows(&self) -> usize {
        self.dim().0
    }

    fn num_columns(&self) -> usize {
        self.dim().1
    }
}

pub fn of(num_rows: usize, num_columns: usize) -> Matrix {
    Matrix::from_elem((num_rows, num_columns), Initial)
}

pub fn from_elements<V: FixedInitializer<Elem=Element>>(elements: &[V]) -> Matrix
    where V: Clone {
    arr2(&elements)
}

#[cfg(test)]
mod tests {
    use super::{Columnar};
    use crate::matrix;
    use crate::matrix::Element::{Start, Substitution};

    #[test]
    fn test_scores() {
        let start = Start;
        assert_eq!(
            start.score(),
            0.0
        );

        let substitution = Substitution(1.0);
        assert_eq!(
            substitution.score(),
            1.0
        );
    }

    #[test]
    fn test_dimensions() {
        let mtx = matrix::of(3, 2);
        assert_eq!(
            mtx.num_rows(),
            3
        );
        assert_eq!(
            mtx.num_columns(),
            2
        );
    }
}