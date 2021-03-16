use std::collections::VecDeque;
use crate::element::{FScore, Op};
use crate::matrix::Idx;
use core::iter;

pub const GAP: char = '_';

#[derive(Debug, PartialEq)]
pub struct Anchor {
    pub idx: Idx,
    pub op: Op,
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
    pub anchors: VecDeque<Anchor>,
}

impl Alignment {
    pub fn from(subject: &str, reference: &str, score: FScore) -> Self {
        Alignment {
            score,
            anchors: to_anchors(subject, reference),
        }
    }

    pub fn print(&self, r: &str, s: &str) {
        let rs: Vec<char> = r.chars().collect();
        let ss: Vec<char> = s.chars().collect();
        self.anchors
            .iter()
            .skip(1)
            .map(|a| match a.op {
                Op::START => ('_', '_'),
                Op::MATCH => (rs[a.idx.1 - 1], ss[a.idx.0 - 1]),
                Op::INSERT => (GAP, ss[a.idx.0 - 1]),
                Op::DELETE => (rs[a.idx.1 - 1], GAP)
            })
            .for_each(|p| println!("{:?} {:?}", p.0, p.1));
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
    subject.chars()
        .zip(reference.chars())
        .map(move |(s, r)|
            Anchor::from(
                inc.with(s, r),
                op(s, r),
            )
        )
}

fn op(s: char, r: char) -> Op {
    match (s, r) {
        (GAP, _) => Op::DELETE,
        (_, GAP) => Op::INSERT,
        _ => Op::MATCH
    }
}

struct IdxIncrementer {
    s_inc: usize,
    r_inc: usize,
}

impl IdxIncrementer {
    const START: Self = Self { r_inc: 0, s_inc: 0 };

    fn with(&mut self, s: char, r: char) -> Idx {
        (
            Self::with_char(&mut self.s_inc, s),
            Self::with_char(&mut self.r_inc, r)
        )
    }

    fn with_char(i: &mut usize, c: char) -> usize {
        if c != GAP {
            *i += 1;
        }
        *i
    }
}

