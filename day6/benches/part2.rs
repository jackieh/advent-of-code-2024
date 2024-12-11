use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::Array2;

use day6::part2;

fn criterion_benchmark(c: &mut Criterion) {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let rows = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let num_rows = rows.len();
    let num_cols = rows.first().unwrap().len();
    let vec = rows.concat().chars().collect::<Vec<_>>();
    let grid = Array2::<char>::from_shape_vec((num_rows, num_cols), vec).unwrap();

    c.bench_function("part 2", |b| b.iter(|| part2(&grid)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
