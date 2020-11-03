use std::str::Bytes;

pub struct SeqIterator<'a> {
    bytes: Bytes<'a>
}

impl SeqIterator<'_> {
    pub fn from(seq: &str) -> SeqIterator {
        SeqIterator { bytes: seq.bytes() }
    }

    pub fn next_char(&mut self) -> char {
        self.bytes.next().unwrap() as char
    }
}