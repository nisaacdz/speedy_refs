use criterion::{criterion_group, criterion_main, Criterion};

const N: usize = 100;

pub struct Dummy(pub Vec<std::sync::Arc<Self>>);

fn clone<T: Clone>(mut a: T) {
    for _ in 0..N {
        a = a.clone();
    }
}

#[allow(unused)]
fn rc(c: &mut Criterion) {
    let std = std::rc::Rc::new(2);
    let mine = speedy_refs::rc::Rc::new(2);
    c.bench_function("MY-RC", |b| b.iter(|| clone(mine.clone())));
    c.bench_function("STD-RC", |b| b.iter(|| clone(std.clone())));
}

#[allow(unused)]
fn arc(c: &mut Criterion) {
    let std = std::sync::Arc::new(2);
    let mine = speedy_refs::arc::Arc::new(2);
    c.bench_function("STD-ARC", |b| b.iter(|| clone(std.clone())));
    c.bench_function("MY-ARC", |b| b.iter(|| clone(mine.clone())));
}

criterion_group!(b1, rc);
criterion_main!(b1);
