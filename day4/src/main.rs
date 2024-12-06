use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::chain;

use ndarray::Array2;

fn part1(grid: &Array2<char>) -> u32 {
    let (num_rows, num_cols) = grid.dim();

    fn find_xmas(state: &mut Option<char>, c: &char) -> Option<u32> {
        let found_xmas = match c {
            'X' => {
                *state = Some('X');
                false
            }
            'M' => {
                *state = match state {
                    Some('X') => Some('M'),
                    _ => None,
                };
                false
            }
            'A' => {
                *state = match state {
                    Some('M') => Some('A'),
                    _ => None,
                };
                false
            }
            'S' => {
                matches!(state.take(), Some('A'))
            }
            _ => false,
        };

        Some(found_xmas as u32)
    }

    let axis_count = chain!(grid.rows().into_iter(), grid.columns().into_iter())
        .map(|view| {
            let forwards = view.iter().scan(None, find_xmas).sum::<u32>();
            let backwards = view.iter().rev().scan(None, find_xmas).sum::<u32>();
            forwards + backwards
        })
        .sum::<u32>();

    let diagonal_count = chain!(
        // Forward diagonals
        (0..(num_rows + num_cols - 1)).map(|start| {
            (0..num_rows)
                .filter_map(|r| {
                    let c = num_cols as i32 - 1 - start as i32 + r as i32;
                    ((0..num_cols as i32).contains(&c)).then(|| grid[[r, c as usize]])
                })
                .collect::<Vec<_>>()
        }),
        // Backward diagonals
        (0..(num_rows + num_cols - 1)).map(|start| {
            (0..num_rows)
                .filter_map(|r| {
                    let c = start as i32 - r as i32;
                    ((0..num_cols as i32).contains(&c)).then(|| grid[[r, c as usize]])
                })
                .collect::<Vec<_>>()
        }),
    )
    .map(|diag| {
        let forwards = diag.iter().scan(None, find_xmas).sum::<u32>();
        let backwards = diag.iter().rev().scan(None, find_xmas).sum::<u32>();
        forwards + backwards
    })
    .sum::<u32>();

    axis_count + diagonal_count
}

fn part2(grid: &Array2<char>) -> u32 {
    let (num_rows, num_cols) = grid.dim();

    let is_ms = |a, b| matches!((a, b), ('M', 'S') | ('S', 'M'));

    let is_xmas = |r, c| {
        grid[[r, c]] == 'A'
            && is_ms(grid[[r - 1, c - 1]], grid[[r + 1, c + 1]])
            && is_ms(grid[[r - 1, c + 1]], grid[[r + 1, c - 1]])
    };

    (1..(num_rows - 1))
        .map(|r| {
            (1..(num_cols - 1))
                .map(|c| is_xmas(r, c) as u32)
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn main() {
    let file = File::open("day4/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let rows = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let num_rows = rows.len();
    let num_cols = rows.first().unwrap().len();
    let vec = rows.concat().chars().collect::<Vec<_>>();
    let grid = Array2::<char>::from_shape_vec((num_rows, num_cols), vec).unwrap();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
