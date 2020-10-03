struct Aligner<'a> {
    reference: &'a str
}

impl<'a> Aligner<'a> {
    fn from(reference: &'a str) -> Self {
        Aligner { reference }
    }

    fn align(&self, subject: &'a str) -> Alignment {
        Alignment::from(self.reference, subject, subject.len() as f64)
    }
}

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


#[cfg(test)]
mod tests {
    use super::{Aligner};

    #[test]
    fn test_match() {
        let aligner = Aligner::from("AGCT");
        let alignment = aligner.align("AGCT");
        assert_eq!(alignment.reference, "AGCT");
        assert_eq!(alignment.subject, "AGCT");
        assert_eq!(alignment.score, 4.0);
    }

    #[test]
    fn test_mismatch() {
        let aligner = Aligner::from("AGCT");
        let alignment = aligner.align("AGAT");
        assert_eq!(alignment.reference, "AGCT");
        assert_eq!(alignment.subject, "AGAT");
        assert_eq!(alignment.score, 2.0);
    }
}