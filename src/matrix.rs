use ndarray::{Array2, FixedInitializer, arr2};
use crate::matrix::Element::{Start, Insertion, Deletion, Substitution, Initial};
use std::process::Output;
use std::ops::Add;

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

impl Add<FScore> for Element {
    type Output = FScore;

    fn add(self, rhs: FScore) -> Self::Output {
        self.score() + rhs
    }
}

pub type Idx = (usize, usize);

pub trait Columnar {
    fn of(num_rows: usize, num_columns: usize) -> Self;
    fn num_rows(&self) -> usize;
    fn num_columns(&self) -> usize;
}

pub type Matrix = Array2<Element>;

impl Columnar for Matrix {
    fn of(num_rows: usize, num_columns: usize) -> Self {
        Matrix::from_elem((num_rows, num_columns), Initial)
    }

    fn num_rows(&self) -> usize {
        self.dim().0
    }

    fn num_columns(&self) -> usize {
        self.dim().1
    }
}

pub fn move_back(element: &Element, position: Idx) -> Idx {
    let (row, column) = position;
    match element {
        Substitution(_) => (row - 1, column - 1),
        Insertion(_) => (row - 1, column),
        Deletion(_) => (row, column - 1),
        _ => unreachable!()
    }
}

pub fn from_elements<V>(elements: &[V]) -> Matrix
    where V: Clone + FixedInitializer<Elem=Element> {
    arr2(&elements)
}

#[cfg(test)]
mod tests {
    use crate::matrix::{Columnar, Matrix};
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
        let mtx = Matrix::of(3, 2);
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