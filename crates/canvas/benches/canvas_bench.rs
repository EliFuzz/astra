use criterion::{Criterion, criterion_group, criterion_main};

use astra_canvas::canvas::CanvasDocument;

fn bench_document_creation(c: &mut Criterion) {
    c.bench_function("document_create", |b| {
        b.iter(|| CanvasDocument::default());
    });
}

criterion_group!(benches, bench_document_creation);
criterion_main!(benches);
