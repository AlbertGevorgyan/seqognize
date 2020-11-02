#[macro_use]
extern crate criterion;

use seqognize::aligner::Aligner;
use seqognize::nt_aligner::{GlobalNtAligner, NtAlignmentConfig};
use criterion::Criterion;

fn nt_alignment_benchmark(c: &mut Criterion) {
    let aligner: GlobalNtAligner = GlobalNtAligner {
        config: NtAlignmentConfig {
            match_score: 1.0,
            mismatch_penalty: -1.0,
            subject_gap_penalty: -1.0,
            reference_gap_penalty: -1.0,
        }
    };
    c.bench_function("NT alignment", |b| b.iter(|| aligner.align("ACGTACT", "ACTACGT")));
}

criterion_group!(nt_alignment, nt_alignment_benchmark);
criterion_main!(nt_alignment);
