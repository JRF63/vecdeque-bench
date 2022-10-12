use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use std::collections::VecDeque;
use vecdeque_bench::Deque as VecDeque;

fn criterion_benchmarks(c: &mut Criterion) {
    c.bench_function("bench_new", |b| {
        b.iter(|| {
            let ring: VecDeque<i32> = VecDeque::new();
            black_box(ring);
        })
    });

    c.bench_function("bench_grow_1025", |b| {
        b.iter(|| {
            let mut deq = VecDeque::new();
            for i in 0..1025 {
                deq.push_front(i);
            }
            black_box(deq);
        })
    });

    let ring: VecDeque<_> = (0..1000).collect();
    c.bench_function("bench_iter_1000", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &i in &ring {
                sum += i;
            }
            black_box(sum);
        })
    });

    let mut ring: VecDeque<_> = (0..1000).collect();
    c.bench_function("bench_mut_iter_1000", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in &mut ring {
                sum += *i;
            }
            black_box(sum);
        })
    });

    let ring: VecDeque<_> = (0..1000).collect();
    c.bench_function("bench_try_fold", |b| {
        b.iter(|| black_box(ring.iter().try_fold(0, |a, b| Some(a + b))))
    });
}

criterion_group!(benches, criterion_benchmarks);
criterion_main!(benches);
