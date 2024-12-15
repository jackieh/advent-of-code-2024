use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use auto_ops::*;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Clone, Copy)]
struct Coordinates {
    r: i32,
    c: i32,
}

impl Coordinates {
    fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    fn out_of_bounds(&self, num_rows: usize, num_cols: usize) -> bool {
        let num_rows = num_rows as i32;
        let num_cols = num_cols as i32;
        self.r < 0 || self.c < 0 || self.r >= num_rows || self.c >= num_cols
    }
}

impl_op!(+ |lhs: Coordinates, rhs: Coordinates| -> Coordinates {
    Coordinates::new(lhs.r + rhs.r, lhs.c + rhs.c)
});
impl_op!(+= |lhs: &mut Coordinates, rhs: Coordinates| {
    lhs.r += rhs.r;
    lhs.c += rhs.c;
});
impl_op!(-|lhs: Coordinates, rhs: Coordinates| -> Coordinates {
    Coordinates::new(lhs.r - rhs.r, lhs.c - rhs.c)
});
impl_op!(-= |lhs: &mut Coordinates, rhs: Coordinates| {
    lhs.r -= rhs.r;
    lhs.c -= rhs.c;
});

fn count_antinodes<F>(grid: &Array2<char>, mark_fn: F) -> usize
where
    F: Fn(&mut Array2<bool>, Coordinates, Coordinates),
{
    let (num_rows, num_cols) = grid.dim();
    let frequencies = grid
        .clone()
        .into_iter()
        .filter(|&c| c != '.')
        .collect::<HashSet<char>>();

    let mut antinodes =
        Array2::<bool>::from_shape_vec((num_rows, num_cols), vec![false; grid.len()]).unwrap();
    for frequency in frequencies {
        for (pos1, pos2) in (0..grid.len())
            .filter_map(|i| {
                let r = i / num_cols;
                let c = i % num_cols;
                (grid[[r, c]] == frequency).then_some(Coordinates::new(r as i32, c as i32))
            })
            .tuple_combinations()
        {
            mark_fn(&mut antinodes, pos1, pos2);
        }
    }

    antinodes.into_iter().filter(|&a| a).count()
}

pub fn part1(grid: &Array2<char>) -> usize {
    let (num_rows, num_cols) = grid.dim();

    let mark_antinodes =
        move |antinodes: &mut Array2<bool>, pos1: Coordinates, pos2: Coordinates| {
            let vector = pos2 - pos1;
            for candidate in [pos1 - vector, pos2 + vector] {
                if !candidate.out_of_bounds(num_rows, num_cols) {
                    antinodes[[candidate.r as usize, candidate.c as usize]] = true;
                }
            }
        };

    count_antinodes(grid, mark_antinodes)
}

pub fn part2(grid: &Array2<char>) -> usize {
    let (num_rows, num_cols) = grid.dim();

    let mark_antinodes =
        move |antinodes: &mut Array2<bool>, pos1: Coordinates, pos2: Coordinates| {
            let vector = pos2 - pos1;
            let mut candidate = pos1;
            while !candidate.out_of_bounds(num_rows, num_cols) {
                antinodes[[candidate.r as usize, candidate.c as usize]] = true;
                candidate -= vector;
            }
            candidate = pos2;
            while !candidate.out_of_bounds(num_rows, num_cols) {
                antinodes[[candidate.r as usize, candidate.c as usize]] = true;
                candidate += vector;
            }
        };

    count_antinodes(grid, mark_antinodes)
}

fn main() {
    let file = File::open("day8/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let rows = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let num_rows = rows.len();
    let num_cols = rows.first().unwrap().len();
    let vec = rows.concat().chars().collect::<Vec<_>>();
    let grid = Array2::<char>::from_shape_vec((num_rows, num_cols), vec).unwrap();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
