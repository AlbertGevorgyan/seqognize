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
    let mut inc = IdxIncrementer::START;
    reference.chars()
        .zip(subject.chars())
        .map(move |(r, s)|
            Anchor::from(
                inc.with(r, s),
                op(r, s),
            )
        )
}

fn op(r: char, s: char) -> Op {
    match (r, s) {
        (GAP, _) => Op::INSERT,
        (_, GAP) => Op::DELETE,
        _ => Op::MATCH
    }
}

struct IdxIncrementer {
    s_inc: Incrementer,
    r_inc: Incrementer,
}

impl IdxIncrementer {
    const START: Self = Self { r_inc: Incrementer::START, s_inc: Incrementer::START };

    fn with(&mut self, s: char, r: char) -> Idx {
        (
            self.r_inc.with(r),
            self.s_inc.with(s),
        )
    }
}

struct Incrementer {
    i: usize
}

impl Incrementer {
    const START: Self = Self { i: 0 };

    fn with(&mut self, c: char) -> usize {
        if c != GAP {
            self.i += 1;
        }
        self.i
    }
}

