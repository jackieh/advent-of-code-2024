use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn part1(left: &[u32], right: &[u32]) -> u32 {
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    left.sort();
    right.sort();
    zip(left, right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part2(left: &[u32], right: &[u32]) -> u32 {
    let mut counts = HashMap::new();
    for elem in right {
        *counts.entry(elem).or_insert(0u32) += 1;
    }
    left.iter()
        .map(|elem| {
            let count = counts.get(elem).unwrap_or(&0);
            elem * count
        })
        .sum()
}

fn main() {
    let file = File::open("day1/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let (left, right): (Vec<u32>, Vec<u32>) = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut split = line.split_whitespace();
            let left = split.next().unwrap().parse::<u32>().unwrap();
            let right = split.next().unwrap().parse::<u32>().unwrap();
            (left, right)
        })
        .unzip();

    println!("Part 1: {}", part1(&left, &right));
    println!("Part 2: {}", part2(&left, &right));
}
