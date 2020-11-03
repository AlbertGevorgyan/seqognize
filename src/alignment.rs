use std::collections::VecDeque;
use crate::matrix::FScore;

pub const GAP: char = '_';

#[derive(Debug, PartialEq)]
pub struct Alignment {
    subject: String,
    reference: String,
    pub score: f64,
}

impl Alignment {
    pub fn from(subject: String, reference: String, score: f64) -> Self {
        Alignment { reference, subject, score }
    }
}

pub struct AlignmentBuilder {
    aligned_subject: VecDeque<u8>,
    aligned_reference: VecDeque<u8>,
}

pub fn builder(capacity: usize) -> AlignmentBuilder {
    AlignmentBuilder {
        aligned_subject: VecDeque::with_capacity(capacity),
        aligned_reference: VecDeque::with_capacity(capacity),
    }
}

impl AlignmentBuilder {
    pub fn prepend_to_subject(&mut self, symbol: u8) {
        self.aligned_subject.push_front(symbol)
    }

    pub fn prepend_to_reference(&mut self, symbol: u8) {
        self.aligned_reference.push_front(symbol)
    }

    pub fn build(self, score: FScore) -> Alignment {
        Alignment::from(
            String::from_utf8(Vec::from(self.aligned_subject)).unwrap(),
            String::from_utf8(Vec::from(self.aligned_reference)).unwrap(),
            score,
        )
    }
}
