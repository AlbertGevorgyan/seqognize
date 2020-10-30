use crate::alignment_mtx;
use ndarray::{Array2, arr2, FixedInitializer};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pointer {
    LEFT,
    SUBST,
    UP,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Element {
    pub score: f64,
    pub pointer: Pointer,
}

pub fn element(score: f64, pointer: Pointer) -> Element {
    Element { score, pointer }
}

impl Element {
    pub fn plus(&self, cost: f64) -> Self {
        element(self.score + cost, self.pointer)
    }

    pub fn minus(&self, cost: f64) -> Self {
        element(self.score - cost, self.pointer)
    }
}

pub trait Mtx {
    fn num_rows(&self) -> usize;
    fn num_columns(&self) -> usize;
}

pub type AlignmentMtx = Array2<Element>;

impl Mtx for AlignmentMtx {
    fn num_rows(&self) -> usize {
        self.dim().0
    }

    fn num_columns(&self) -> usize {
        self.dim().1
    }
}

pub const INITIAL_ELEMENT: Element = Element { score: 0.0, pointer: Pointer::SUBST };
const OUT_OF_BOUNDS_MSG: &'static str = "Index is out of bounds.";

pub fn of(num_rows: usize, num_columns: usize) -> AlignmentMtx {
    AlignmentMtx::from_elem((num_rows, num_columns), alignment_mtx::INITIAL_ELEMENT)
}

pub fn from_elements<V: FixedInitializer<Elem=Element>>(elements: &[V]) -> AlignmentMtx
    where V: Clone {
    arr2(&elements)
}

#[cfg(test)]
mod tests {
    use super::{Mtx, Pointer, Element};
    use crate::alignment_mtx::{of, element};

    #[test]
    fn test_element() {
        let score = 0.0;
        let pointer: Pointer = Pointer::LEFT;
        let mut el = Element { score, pointer };
        assert_eq!(el, Element { score: 0.0, pointer: Pointer::LEFT });
        assert_eq!(el.plus(1.0), Element { score: 1.0, pointer: Pointer::LEFT });
        assert_eq!(el.minus(1.0), Element { score: -1.0, pointer: Pointer::LEFT });
    }

    #[test]
    fn test_dimensions() {
        let mtx = of(3, 2);
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