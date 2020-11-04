use std::str::Bytes;
use std::iter::{Successors, successors};

pub struct SeqIterator<'a> {
    bytes: Bytes<'a>
}

impl SeqIterator<'_> {
    pub fn from(seq: &str) -> SeqIterator {
        SeqIterator { bytes: seq.bytes() }
    }

    pub fn next_char(&mut self) -> char {
        self.bytes.next().unwrap() as char
    }
}

pub fn accumulate<S>(size: usize, supplier: S) -> Successors<f64, impl FnMut(&f64) -> Option<f64>>
    where S: Fn(usize) -> f64 {
    let mut range = 0..size;
    successors(
        Some(0.0),
        move |acc| range.next().map(|n| *acc + supplier(n)),
    )
}