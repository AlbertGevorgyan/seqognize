struct Aligner<'a> {
    reference: &'a str
}

impl<'a> Aligner<'a> {
    fn from(reference: &'a str) -> Self {
        Aligner { reference }
    }

    fn align(&self, subject: &'a str) -> Alignment {
        Alignment::from(self.reference, subject)
    }
}

struct Alignment<'a> {
    reference: &'a str,
    subject: &'a str,
}

impl<'a> Alignment<'a> {
    fn from(reference: &'a str, subject: &'a str) -> Self {
        Alignment { reference, subject }
    }
}


#[cfg(test)]
mod tests {
    use super::{Aligner, Alignment};

    #[test]
    fn test_match() {
        let aligner = Aligner::from("AGCT");
        let alignment = aligner.align("AGCT");
        assert_eq!(alignment.reference, "AGCT");
        assert_eq!(alignment.subject, "AGCT");
    }

    #[test]
    fn test_mismatch() {
        let aligner = Aligner::from("AGCT");
        let alignment = aligner.align("AGAT");
        assert_eq!(alignment.reference, "AGCT");
        assert_eq!(alignment.subject, "AGAT");
    }
}