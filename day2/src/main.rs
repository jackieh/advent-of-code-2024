use std::{
    cmp::PartialOrd,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn part1(reports: &[Vec<u32>]) -> u32 {
    fn all_adjacent<F>(report: &[u32], f: F) -> bool
    where
        F: Fn(&u32, &u32) -> bool,
    {
        report
            .iter()
            .tuple_windows()
            .all(|(left, right)| f(left, right) && left.abs_diff(*right) <= 3)
    }

    reports
        .iter()
        .filter(|r| all_adjacent(r, u32::lt) || all_adjacent(r, u32::gt))
        .count() as u32
}

fn part2(reports: &[Vec<u32>]) -> u32 {
    fn all_but_one_adjacent<F>(report: &[u32], f: F) -> bool
    where
        F: Fn(&u32, &u32) -> bool,
    {
        let is_safe = |left, right| f(&left, &right) && left.abs_diff(right) <= 3;

        let failures = report
            .iter()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (&left, &right))| !is_safe(left, right))
            .collect::<Vec<_>>();

        match failures[..] {
            [] => true,
            [(i, _)] => {
                i == 0
                    || i == report.len() - 2
                    || is_safe(report[i - 1], report[i + 1])
                    || is_safe(report[i], report[i + 2])
            }
            [(i, (&left, _)), (j, (_, &right))] => i + 1 == j && is_safe(left, right),
            _ => false,
        }
    }

    reports
        .iter()
        .filter(|r| all_but_one_adjacent(r, u32::lt) || all_but_one_adjacent(r, u32::gt))
        .count() as u32
}

fn main() {
    let file = File::open("day2/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let reports = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&reports));
    println!("Part 2: {}", part2(&reports));
}
