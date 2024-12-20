use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ndarray::Array2;

use day6::{part1, part2};

fn main() {
    let file = File::open("day6/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let rows = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let num_rows = rows.len();
    let num_cols = rows.first().unwrap().len();
    let vec = rows.concat().chars().collect::<Vec<_>>();
    let grid = Array2::<char>::from_shape_vec((num_rows, num_cols), vec).unwrap();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
