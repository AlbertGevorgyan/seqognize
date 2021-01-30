#![allow(dead_code)]

use std::env;
use seqognize::nt_aligner::{GlobalNtAligner, NtAlignmentConfig};
use seqognize::aligner::Aligner;

mod matrix;
mod aligner;
mod config;
mod nt_aligner;
mod alignment;
mod iterators;
mod element;

fn main() {
    let args: Vec<String> = env::args().collect();

    let subject = args.get(1).unwrap();
    let reference = args.get(2).unwrap();

    let aligner: GlobalNtAligner = GlobalNtAligner {
        config: NtAlignmentConfig {
            match_score: 1.0,
            mismatch_penalty: -1.0,
            subject_gap_penalty: -1.0,
            reference_gap_penalty: -1.0,
        }
    };

    let alignment = aligner.align(subject, reference);

    println!("{:?}", alignment);
}
