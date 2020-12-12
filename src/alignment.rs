use std::collections::VecDeque;
use crate::element::{FScore, Pointer};
use crate::matrix::Idx;

pub const GAP: char = '_';

#[derive(Debug, PartialEq)]
struct Anchor {
    idx: Idx,
    pointer: Pointer,
}

impl Anchor {
    fn from(idx: Idx, pointer: Pointer) -> Self {
        Anchor { idx, pointer }
    }
}

fn to_anchors(subject: &str, reference: &str) -> VecDeque<Anchor> {
    let mut idx = MutableIdx::start();
    reference.chars()
        .zip(subject.chars())
        .map(|(r, s)| to_anchor(&mut idx, r, s))
        .collect()
}

fn to_anchor(idx: &mut MutableIdx, r: char, s: char) -> Anchor {
    match (r, s) {
        (GAP, _) => Anchor::from(
            idx.step(1, 0),
            Pointer::UP,
        ),
        (_, GAP) => Anchor::from(
            idx.step(0, 1),
            Pointer::LEFT,
        ),
        _ => Anchor::from(
            idx.step(1, 1),
            Pointer::DIAGONAL,
        )
    }
}

struct MutableIdx {
    r: usize,
    s: usize,
}

impl MutableIdx {
    fn start() -> Self {
        MutableIdx { r: 0, s: 0 }
    }

    fn step(&mut self, r_step: usize, s_step: usize) -> Idx {
        self.r += r_step;
        self.s += s_step;
        (self.r, self.s)
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

    pub fn take(&mut self, pointer: Pointer, idx: Idx) {
        self.anchors.push_front(Anchor { idx, pointer })
    }

    pub fn build(self, score: FScore) -> Alignment {
        Alignment {
            score,
            anchors: self.anchors,
        }
    }
}
