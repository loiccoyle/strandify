use std::iter::zip;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use strandify::pather::{Pather, PatherConfig};
use strandify::peg::Peg;
use strandify::utils;

fn input_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("input.jpg")
}

fn create_pather() -> Pather {
    let image = image::open(input_file()).unwrap();
    let image_gray = image.to_luma8();

    let (pegs_x, pegs_y) = utils::rectangle_coords(
        image.width() - 12,
        image.height() - 12,
        (image.width() / 2, image.height() / 2),
        288,
    );
    let pegs = zip(pegs_x, pegs_y)
        .enumerate()
        .map(|(i, (x, y))| Peg::new(x, y, i as u16))
        .collect::<Vec<_>>();

    let config = PatherConfig {
        iterations: 100,
        beam_width: 5,
        ..Default::default()
    };
    let mut pather = Pather::new(image_gray, pegs, config);
    pather.populate_line_cache().unwrap();
    pather
}

fn benchmark_beam_search(c: &mut Criterion) {
    let pather = create_pather();
    c.bench_function("compute_beam", |b| {
        b.iter(|| {
            let result = black_box(pather.compute_beam());
            assert!(result.is_ok());
        })
    });
}

fn benchmark_greedy(c: &mut Criterion) {
    let pather = create_pather();
    c.bench_function("compute_greedy", |b| {
        b.iter(|| {
            let result = black_box(pather.compute_greedy());
            assert!(result.is_ok());
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = benchmark_greedy, benchmark_beam_search
}
criterion_main!(benches);
