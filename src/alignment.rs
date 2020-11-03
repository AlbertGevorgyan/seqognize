use std::collections::VecDeque;
use crate::matrix::{FScore, Element};
use std::iter::Rev;
use std::str::Bytes;
use crate::matrix::Element::{Substitution, Insertion, Deletion};

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

impl AlignmentBuilder<'_> {

    pub fn from<'a>(subject: &'a str, reference: &'a str) -> AlignmentBuilder<'a> {
        let capacity = subject.len() + reference.len();
        AlignmentBuilder {
            subject_builder: AlignedSequenceBuilder::from(subject, capacity),
            reference_builder: AlignedSequenceBuilder::from(reference, capacity),
        }
    }

    pub fn build(self, score: FScore) -> Alignment {
        Alignment::from(
            self.subject_builder.build(),
            self.reference_builder.build(),
            score,
        )
    }

    pub fn handle(&mut self, element: &Element) {
        match element {
            Substitution(_) => self.take_both(),
            Insertion(_) => self.gap_reference(),
            Deletion(_) => self.gap_subject(),
            _ => unreachable!()
        };
    }

    fn take_both(&mut self) {
        self.subject_builder.take();
        self.reference_builder.take();
    }

    fn gap_subject(&mut self) {
        self.subject_builder.gap();
        self.reference_builder.take();
    }

    fn gap_reference(&mut self) {
        self.subject_builder.take();
        self.reference_builder.gap();
    }
}

pub struct AlignedSequenceBuilder<'a> {
    source: Rev<Bytes<'a>>,
    aligned: VecDeque<u8>,
}

impl AlignedSequenceBuilder<'_> {

    pub fn from(sequence: &str, capacity: usize) -> AlignedSequenceBuilder {
        AlignedSequenceBuilder {
            source: sequence.bytes().rev(),
            aligned: VecDeque::with_capacity(capacity),
        }
    }

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