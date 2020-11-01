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
