use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use image2::*;
use image_processor::cli::Command;

/// Benchmark the image processing functions and group their results
fn bench_image_processor(c: &mut Criterion) {
    let num_threads_permutations: Vec<usize> = vec![1, 2, 4, 8];
    let image = Image::<u8, Rgb>::open("resources/fruit.jpg").unwrap();
    let mut group = c.benchmark_group("Image Processor");
    for num_threads in num_threads_permutations {   
        let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
        group.bench_with_input(BenchmarkId::new("Brightness", num_threads), &num_threads, |b, &_num_threads| {
            b.iter(|| {
                let mut loop_image = image.clone();
                pool.install(|| image_processor::image_processor::process_image(&mut loop_image, Command::Brightness { amount: 50.0 }));
            })
        });
        group.bench_with_input(BenchmarkId::new("Contrast", num_threads), &num_threads, |b, &_num_threads| {
            b.iter(|| {
                let mut loop_image = image.clone();
                pool.install(|| image_processor::image_processor::process_image(&mut loop_image, Command::Contrast { factor: 1.5 }));
            })
        });
        group.bench_with_input(BenchmarkId::new("Gamma", num_threads), &num_threads, |b, &_num_threads| {
            b.iter(|| {
                let mut loop_image = image.clone();
                pool.install(|| image_processor::image_processor::process_image(&mut loop_image, Command::Gamma { amount: 1.5 }));
            })
        });
        group.bench_with_input(BenchmarkId::new("Invert", num_threads), &num_threads, |b, &_num_threads| {
            b.iter(|| {
                let mut loop_image = image.clone();
                pool.install(|| image_processor::image_processor::process_image(&mut loop_image, Command::Invert));
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_image_processor);
criterion_main!(benches);