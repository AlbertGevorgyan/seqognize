use array2d::Array2D;
use crate::alignment_mtx;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pointer {
    LEFT,
    SUBST,
    UP,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub score: f64,
    pub pointer: Pointer,
}

pub fn element(score: f64, pointer: Pointer) -> Element {
    Element { score, pointer }
}

pub trait ScoreMtx {
    fn put(&mut self, row: usize, column: usize, score: f64, pointer: Pointer);
}

pub type AlignmentMtx = Array2D<Element>;

impl ScoreMtx for AlignmentMtx {
    fn put(&mut self, row: usize, column: usize, score: f64, pointer: Pointer) {
        self.set(row, column, element(score, pointer));
    }
}

pub const INITIAL_ELEMENT: Element = Element { score: 0.0, pointer: Pointer::SUBST };
const OUT_OF_BOUNDS_MSG: &'static str = "Index is out of bounds.";

pub fn of(num_rows: usize, num_columns: usize) -> AlignmentMtx {
    AlignmentMtx::filled_with(alignment_mtx::INITIAL_ELEMENT, num_rows, num_columns)
}


#[cfg(test)]
mod tests {
    use super::{Pointer, Element};

    #[test]
    fn test_element() {
        let score = 0.0;
        let pointer: Pointer = Pointer::LEFT;
        let el = Element { score, pointer };
        assert_eq!(el, Element { score: 0.0, pointer: Pointer::LEFT });
        assert_ne!(el, Element { score: 0.01, pointer: Pointer::LEFT });
        assert_ne!(el, Element { score: 0.01, pointer: Pointer::UP });
    }
}