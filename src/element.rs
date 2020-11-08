use std::ops::Add;

pub type FScore = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pointer {
    UNDEF,
    UP,
    DIAGONAL,
    LEFT,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Element {
    pub pointer: Pointer,
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
        Element { pointer: Pointer::UNDEF, score: 0.0 }
    }
}

