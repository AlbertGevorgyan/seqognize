use ndarray::{Array2, FixedInitializer, arr2};
use crate::element::{Pointer, Element, INITIAL};

pub type Idx = (usize, usize);

pub trait Columnar {
    fn of(num_rows: usize, num_columns: usize) -> Self;
    fn num_rows(&self) -> usize;
    fn num_columns(&self) -> usize;
}

pub type Matrix = Array2<Element>;

impl Columnar for Matrix {
    fn of(num_rows: usize, num_columns: usize) -> Self {
        Matrix::from_elem((num_rows, num_columns), INITIAL)
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
    match element.pointer {
        Pointer::DIAGONAL => (row - 1, column - 1),
        Pointer::UP => (row - 1, column),
        Pointer::LEFT => (row, column - 1),
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