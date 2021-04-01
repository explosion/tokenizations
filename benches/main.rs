use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokenizations;

fn get_alignments(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_alignments");
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
    let u = black_box(vec![
        "zzz",
        "zzzzzz",
        "ppppp",
        "pppp",
        "ppppppp",
        "ppppppppppppppp",
    ]);

    group.bench_function("handmade short", |b| {
        b.iter(|| tokenizations::get_alignments(&s, &t))
    });

    let n = black_box(100);
    let s_long = s.repeat(n);
    let t_long = t.repeat(n);
    let u_long = u.repeat(n);
    group.bench_function("handmade long", |b| {
        b.iter(|| tokenizations::get_alignments(&s_long, &t_long))
    });

    group.bench_function("identical short", |b| {
        b.iter(|| tokenizations::get_alignments(&s, &s))
    });

    group.bench_function("identical long", |b| {
        b.iter(|| tokenizations::get_alignments(&s_long, &s_long))
    });

    group.bench_function("completely different short", |b| {
        b.iter(|| tokenizations::get_alignments(&s, &u))
    });

    group.bench_function("completely different long", |b| {
        b.iter(|| tokenizations::get_alignments(&s_long, &u_long))
    });
    group.finish()
}

criterion_group!(benches, get_alignments);
criterion_main!(benches);
