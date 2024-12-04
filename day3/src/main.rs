use std::fs;

use regex::Regex;

fn part1(input: &str) -> u32 {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            a * b
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    Regex::new(r"don't\(\)|do\(\)|mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .scan(true, |state, c| match c.get(0).unwrap().as_str() {
            "do()" => {
                *state = true;
                Some(0)
            }
            "don't()" => {
                *state = false;
                Some(0)
            }
            _ => Some(
                state
                    .then(|| {
                        let a = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                        let b = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                        a * b
                    })
                    .unwrap_or_default(),
            ),
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("day3/data/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
