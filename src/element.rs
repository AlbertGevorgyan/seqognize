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
pub struct ScoredOp {
    pub op: Op,
    pub score: FScore,
}

impl ScoredOp {
    pub fn from(op: Op, score: FScore) -> ScoredOp {
        ScoredOp { op, score }
    }

    pub fn inf() -> ScoredOp {
        ScoredOp::from(Op::START, NEG_INFINITY)
    }
}

impl Add<FScore> for ScoredOp {
    type Output = FScore;

    fn add(self, rhs: FScore) -> Self::Output {
        self.score + rhs
    }
}

impl Default for ScoredOp {
    fn default() -> Self {
        Self::from(Op::START, 0.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triple {
    pub m: ScoredOp,
    pub x: ScoredOp,
    pub y: ScoredOp,
}

impl Triple {
    pub fn from(m: ScoredOp, x: ScoredOp, y: ScoredOp) -> Triple {
        Triple { m, x, y }
    }
}

impl Default for Triple {
    fn default() -> Self {
        Self::from(
            ScoredOp::default(),
            ScoredOp::default(),
            ScoredOp::default(),
        )
    }
}