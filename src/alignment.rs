#[derive(Debug, PartialEq)]
pub struct Alignment<'a> {
    reference: &'a str,
    subject: &'a str,
    pub(crate) score: f64,
}

impl<'a> Alignment<'a> {
    pub fn from(reference: &'a str, subject: &'a str, score: f64) -> Self {
        Alignment { reference, subject, score }
    }
}
