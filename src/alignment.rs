use std::collections::VecDeque;
use crate::element::{FScore, Element, Pointer};
use std::iter::Rev;
use std::str::Bytes;
use crate::matrix::Idx;

pub const GAP: char = '_';

#[derive(Debug, PartialEq)]
struct Anchor {
    idx: Idx,
    pointer: Pointer,
}

impl Anchor {
    fn from(r_idx: usize, s_idx: usize, pointer: Pointer) -> Self {
        Anchor {
            idx: (r_idx, s_idx),
            pointer,
        }
    }
}

fn to_anchors(subject: &str, reference: &str) -> VecDeque<Anchor> {
    let mut s_idx = 0;
    let mut r_idx = 0;
    reference.chars()
        .zip(subject.chars())
        .map(|(r, s)| {
            let pointer = match (r, s) {
                (GAP, _) => {
                    r_idx += 1;
                    Pointer::UP
                }
                (_, GAP) => {
                    s_idx += 1;
                    Pointer::LEFT
                }
                _ => {
                    r_idx += 1;
                    s_idx += 1;
                    Pointer::DIAGONAL
                }
            };
            Anchor::from(r_idx, s_idx, pointer)
        })
        .collect()
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
