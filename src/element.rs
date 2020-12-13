use std::ops::Add;

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

impl Add<FScore> for Element {
    type Output = FScore;

    fn add(self, rhs: FScore) -> Self::Output {
        self.score + rhs
    }
}

impl Default for Element {
    fn default() -> Self {
        Element { op: Op::START, score: 0.0 }
    }
}

