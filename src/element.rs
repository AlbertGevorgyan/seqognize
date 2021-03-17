use std::ops::Add;
use std::f64::NEG_INFINITY;

pub type FScore = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    START,
    INSERT,
    MATCH,
    DELETE,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Element {
    pub op: Op,
    pub score: FScore,
}

impl Element {
    pub fn from(op: Op, score: FScore) -> Element {
        Element { op, score }
    }

    pub fn inf() -> Element {
        Element::from(Op::START, NEG_INFINITY)
    }
}

impl Add<FScore> for Element {
    type Output = FScore;

    fn add(self, rhs: FScore) -> Self::Output {
        self.score + rhs
    }
}

impl Default for Element {
    fn default() -> Self {
        Self::from(Op::START, 0.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triple {
    pub m: Element,
    pub x: Element,
    pub y: Element,
}

impl Triple {
    pub fn from(m: Element, x: Element, y: Element) -> Triple {
        Triple { m, x, y }
    }
}

impl Default for Triple {
    fn default() -> Self {
        Self::from(
            Element::default(),
            Element::default(),
            Element::default(),
        )
    }
}