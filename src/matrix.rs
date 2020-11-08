use ndarray::{Array2, FixedInitializer, arr2};
use crate::element::{Pointer, Element};

pub type Idx = (usize, usize);

pub type Matrix = Array2<Element>;

pub fn move_back(element: &Element, position: Idx) -> Idx {
    let (row, column) = position;
    match element.pointer {
        Pointer::DIAGONAL => (row - 1, column - 1),
        Pointer::UP => (row - 1, column),
        Pointer::LEFT => (row, column - 1),
        _ => unreachable!()
    }
}

pub(crate) fn of(num_rows: usize, num_columns: usize) -> Matrix {
    Matrix::from_elem((num_rows, num_columns), Element::default())
}

pub fn from_elements<V>(elements: &[V]) -> Matrix
    where V: Clone + FixedInitializer<Elem=Element> {
    arr2(&elements)
}

