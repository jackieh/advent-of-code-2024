use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_concat(result: u64, operand: u64) -> Option<u64> {
    if result <= operand || result % 10 != operand % 10 {
        None
    } else if operand < 10 {
        Some(result / 10)
    } else {
        is_concat(result / 10, operand / 10)
    }
}

fn is_equation(result: u64, operands: &[u64], has_concat: bool) -> bool {
    match operands {
        [] => unreachable!(),
        &[op] => result == op,
        [tail @ .., op] => {
            let sum = result > *op && is_equation(result - op, tail, has_concat);
            let product = result % op == 0 && is_equation(result / op, tail, has_concat);
            let concat = has_concat
                && is_concat(result, *op)
                    .map(|remainder| is_equation(remainder, tail, has_concat))
                    .unwrap_or_default();
            sum || product || concat
        }
    }
}

fn part1(equations: &[(u64, Vec<u64>)]) -> u64 {
    equations
        .iter()
        .filter_map(|(result, operands)| is_equation(*result, operands, false).then_some(result))
        .sum::<u64>()
}

fn part2(equations: &[(u64, Vec<u64>)]) -> u64 {
    equations
        .iter()
        .filter_map(|(result, operands)| is_equation(*result, operands, true).then_some(result))
        .sum::<u64>()
}

fn main() {
    let file = File::open("day7/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let equations = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut split = line.split_terminator(':');
            let result = split.next().unwrap().trim().parse::<u64>().unwrap();
            let operands = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|op| op.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (result, operands)
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_concat() {
        assert_eq!(is_concat(0, 0), None);
        assert_eq!(is_concat(0, 9), None);
        assert_eq!(is_concat(1, 11), None);
        assert_eq!(is_concat(9, 0), None);
        assert_eq!(is_concat(10, 0), Some(1));
        assert_eq!(is_concat(11, 11), None);
        assert_eq!(is_concat(12, 2), Some(1));
        assert_eq!(is_concat(12345, 5), Some(1234));
        assert_eq!(is_concat(12345, 45), Some(123));
        assert_eq!(is_concat(12345, 2345), Some(1));
        assert_eq!(is_concat(12345, 12345), None);
        assert_eq!(is_concat(12345, 123456), None);
    }
}
