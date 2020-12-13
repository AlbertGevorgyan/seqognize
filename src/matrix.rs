use ndarray::{Array2, FixedInitializer, arr2};
use crate::element::{Op, Element};

pub type Idx = (usize, usize);

pub type Matrix = Array2<Element>;

pub fn of(num_rows: usize, num_columns: usize) -> Matrix {
    Matrix::from_elem((num_rows, num_columns), Element::default())
}

pub fn from_elements<V>(elements: &[V]) -> Matrix
    where V: Clone + FixedInitializer<Elem=Element> {
    arr2(&elements)
}

pub fn move_back(element: &Element, position: Idx) -> Idx {
    let (row, column) = position;
    match element.op {
        Op::MATCH => (row - 1, column - 1),
        Op::INSERT => (row - 1, column),
        Op::DELETE => (row, column - 1),
        _ => unreachable!()
    }
}