use std::str::Bytes;
use std::iter::{successors};
use std::ops::Add;

pub struct SeqIterator<'a> {
    bytes: Bytes<'a>
}

impl SeqIterator<'_> {
    pub fn from(seq: &str) -> SeqIterator {
        SeqIterator { bytes: seq.bytes() }
    }

    pub fn next_byte(&mut self) -> u8 {
        self.bytes.next().unwrap()
    }
}

pub fn accumulate<S, V>(size: usize, supplier: S) -> impl Iterator<Item=V>
    where V: Add<V, Output=V> + Default + Copy,
          S: Fn(usize) -> V {
    let mut range = 0..size;
    successors(
        Some(V::default()),
        move |acc| range.next().map(|n| *acc + supplier(n)),
    )
}