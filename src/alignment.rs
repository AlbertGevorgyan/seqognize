struct Aligner<'a> {
    reference: &'a str
}

impl<'a> Aligner<'a> {
    fn from(reference: &'a str) -> Self {
        Aligner { reference }
    }

    fn align(&self, subject: &'a str) -> Alignment {
        Alignment { reference: self.reference, subject }
    }
}

struct Alignment<'a> {
    reference: &'a str,
    subject: &'a str,
}


#[cfg(test)]
mod tests {
    use super::{Aligner, Alignment};

    #[test]
    fn test_match() {
        let aligner = Aligner::from("ACGT");
        let alignment: Alignment = aligner.align("ACGT");
        assert_eq!(alignment.reference, "ACGT");
        assert_eq!(alignment.subject, "ACGT");
    }
}