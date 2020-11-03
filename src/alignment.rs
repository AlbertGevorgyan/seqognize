use std::collections::VecDeque;
use crate::matrix::FScore;
use std::iter::Rev;
use std::str::Bytes;

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

pub struct AlignmentBuilder<'a> {
    pub subject_builder: AlignedSequenceBuilder<'a>,
    pub reference_builder: AlignedSequenceBuilder<'a>,
}

pub fn builder<'a>(subject: &'a str, reference: &'a str) -> AlignmentBuilder<'a> {
    let capacity = subject.len() + reference.len();
    AlignmentBuilder {
        subject_builder: aligned_seq_builder(subject, capacity),
        reference_builder: aligned_seq_builder(reference, capacity),
    }
}

impl AlignmentBuilder<'_> {
    pub fn build(self, score: FScore) -> Alignment {
        Alignment::from(
            self.subject_builder.build(),
            self.reference_builder.build(),
            score,
        )
    }
}

fn aligned_seq_builder(sequence: &str, capacity: usize) -> AlignedSequenceBuilder {
    AlignedSequenceBuilder {
        source: sequence.bytes().rev(),
        aligned: VecDeque::with_capacity(capacity),
    }
}

pub struct AlignedSequenceBuilder<'a> {
    source: Rev<Bytes<'a>>,
    aligned: VecDeque<u8>,
}

impl AlignedSequenceBuilder<'_> {
    pub fn take(&mut self) {
        self.aligned.push_front(self.source.next().unwrap());
    }

    pub fn gap(&mut self) {
        self.aligned.push_front(GAP as u8);
    }

    pub fn build(self) -> String {
        String::from_utf8(Vec::from(self.aligned)).unwrap()
    }
}