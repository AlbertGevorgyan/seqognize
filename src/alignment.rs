use std::collections::VecDeque;
use crate::element::{FScore, Op};
use crate::matrix::Idx;
use core::iter;

pub const GAP: char = '_';

#[derive(Debug, PartialEq)]
struct Anchor {
    idx: Idx,
    op: Op,
}

impl Anchor {
    const START: Self = Self { idx: (0, 0), op: Op::START };

    fn from(idx: Idx, op: Op) -> Self {
        Anchor { idx, op }
    }
}

#[derive(Debug, PartialEq)]
pub struct Alignment {
    pub score: FScore,
    anchors: VecDeque<Anchor>,
}

impl Alignment {
    pub fn from(subject: &str, reference: &str, score: FScore) -> Self {
        Alignment {
            score,
            anchors: to_anchors(subject, reference),
        }
    }
}

pub struct AlignmentBuilder {
    anchors: VecDeque<Anchor>,
}

impl AlignmentBuilder {
    pub fn new(subject: &str, reference: &str) -> AlignmentBuilder {
        AlignmentBuilder {
            anchors: VecDeque::with_capacity(subject.len() + reference.len()),
        }
    }

    pub fn take(&mut self, op: Op, idx: Idx) {
        self.anchors.push_front(Anchor { idx, op })
    }

    pub fn build(self, score: FScore) -> Alignment {
        Alignment {
            score,
            anchors: self.anchors,
        }
    }
}

fn to_anchors(subject: &str, reference: &str) -> VecDeque<Anchor> {
    iter::once(Anchor::START)
        .chain(from_strings(subject, reference))
        .collect()
}

fn from_strings<'a>(subject: &'a str, reference: &'a str) -> impl Iterator<Item=Anchor> + 'a {
    let mut idx = MutableIdx::START;
    reference.chars()
        .zip(subject.chars())
        .map(move |(r, s)| to_anchor(&mut idx, r, s))
}

fn to_anchor(idx: &mut MutableIdx, r: char, s: char) -> Anchor {
    match (r, s) {
        (GAP, _) => Anchor::from(
            idx.inc(0, 1),
            Op::INSERT,
        ),
        (_, GAP) => Anchor::from(
            idx.inc(1, 0),
            Op::DELETE,
        ),
        _ => Anchor::from(
            idx.inc(1, 1),
            Op::MATCH,
        )
    }
}

struct MutableIdx {
    idx: Idx
}

impl MutableIdx {
    const START: Self = Self { idx: (0, 0) };

    fn inc(&mut self, s_step: usize, r_step: usize) -> Idx {
        self.idx = (self.idx.0 + r_step, self.idx.1 + s_step);
        self.idx
    }
}
