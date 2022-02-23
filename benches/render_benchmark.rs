use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rustyrender::renderer::{self, ComputeEnv};

fn render_harness((w, h): (u32, u32), env: ComputeEnv) {
    let mut pixels = vec![0; (w as usize) * (h as usize) * 3];
    let world = renderer::scene::world_builder::random_scene();

    let _ = renderer::render(env, 1, 10, &world, &mut pixels, (w, h));
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("backend test");
    group.sample_size(50);

    for env in [
        // ComputeEnv::Naive,
        ComputeEnv::Multicore,
        ComputeEnv::SimpleThreaded,
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(env), &env, |b, &env| {
            b.iter(|| render_harness(black_box((1920, 1080)), black_box(env)));
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
