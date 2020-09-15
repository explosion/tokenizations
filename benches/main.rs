use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokenizations;

fn bench_get_alignments(c: &mut Criterion) {
    let s = black_box(vec![
        "asd",
        "asdfasdf",
        "asdfa",
        "-02 t",
        "q2-0t",
        "q -q24t0-q4t2",
    ]);
    let t = black_box(vec![
        "asd",
        "afasdf",
        "0sdfa",
        "-02t",
        "q2---0t",
        "q --:あh4t0-q4t2",
    ]);
    let s = s.repeat(10);
    let t = t.repeat(10);
    c.bench_function("get_alignments", |b| {
        b.iter(|| tokenizations::get_alignments(&s, &t))
    });
}

criterion_group!(benches, bench_get_alignments);
criterion_main!(benches);
