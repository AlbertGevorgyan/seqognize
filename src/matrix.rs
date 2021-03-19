#![allow(dead_code)]

use ndarray::Array2;
use crate::element::{Op, El, ScoredOp};

pub type Idx = (usize, usize);

pub type Matrix = Array2<El>;

pub fn of(num_rows: usize, num_columns: usize) -> Matrix {
    Matrix::from_elem((num_rows, num_columns), El::default())
}

pub fn move_back(element: &ScoredOp, position: Idx) -> Idx {
    let (row, column) = position;
    match element.op {
        Op::MATCH => (row - 1, column - 1),
        Op::INSERT => (row - 1, column),
        Op::DELETE => (row, column - 1),
        _ => unreachable!()
    }
}