use ndarray::{Array2, FixedInitializer, arr2};
use crate::matrix::Element::{Initial, Start, Insertion, Deletion, Substitution};

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
    pub fn score(&self) -> Option<FScore> {
        return match self {
            Initial => None,
            Start => Some(0.0),
            Insertion(score) | Deletion(score) | Substitution(score) => Some(*score),
        };
    }
}

pub fn max_score<'a>(elements: &'a [&Element]) -> Option<&'a Element> {
    if elements.is_empty() {
        None
    } else {
        Some(
            elements.iter()
                .fold(
                    elements[0],
                    |el1, el2| if el1.score().unwrap() > el2.score().unwrap() { el1 } else { el2 },
                )
        )
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
    use super::{Element, Columnar};
    use crate::matrix;

    #[test]
    fn test_scores() {
        let initial = Element::Initial;
        assert_eq!(
            initial.score(),
            None
        );

        let start = Element::Start;
        assert_eq!(
            start.score(),
            Some(0.0)
        );

        let substitution = Element::Substitution(1.0);
        assert_eq!(
            substitution.score(),
            Some(1.0)
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