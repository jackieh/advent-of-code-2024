use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::ControlFlow,
};

trait GetMiddle<T> {
    fn middle(&self) -> Option<&T>;
}

impl<T> GetMiddle<T> for &[T] {
    fn middle(&self) -> Option<&T> {
        self.get(self.len() / 2)
    }
}

struct RulesLoader<'a> {
    lines: &'a mut Box<dyn Iterator<Item = String>>,
}

impl<'a> RulesLoader<'a> {
    fn new(lines: &'a mut Box<dyn Iterator<Item = String>>) -> Self {
        Self { lines }
    }
}

impl Iterator for RulesLoader<'_> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();
        match line.as_deref() {
            Some("") | None => None,
            Some(s) => {
                let mut split = s.split_terminator('|');
                let before = split.next().unwrap().parse::<u32>().unwrap();
                let after = split.next().unwrap().parse::<u32>().unwrap();
                Some((before, after))
            }
        }
    }
}
struct UpdatesLoader<'a> {
    lines: &'a mut Box<dyn Iterator<Item = String>>,
}

impl<'a> UpdatesLoader<'a> {
    fn new(lines: &'a mut Box<dyn Iterator<Item = String>>) -> Self {
        Self { lines }
    }
}

impl Iterator for UpdatesLoader<'_> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            line.split_terminator(',')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
    }
}

fn build_before_rules(rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut result: HashMap<u32, HashSet<u32>> = HashMap::new();
    for &(before, after) in rules {
        result.entry(before).or_default().insert(after);
    }
    result
}

fn build_after_rules(rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut result: HashMap<u32, HashSet<u32>> = HashMap::new();
    for &(before, after) in rules {
        result.entry(after).or_default().insert(before);
    }
    result
}

fn has_correct_ordering(pages: &[u32], before_rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut printed = HashSet::new();
    pages
        .iter()
        .try_for_each(|page| {
            if let Some(after) = before_rules.get(page) {
                if printed.intersection(after).next().is_some() {
                    return ControlFlow::Break(());
                }
            }
            printed.insert(*page);
            ControlFlow::Continue(())
        })
        .is_continue()
}

// Assumes the given graph is acyclic
fn find_leaf(target: u32, graph: &HashMap<u32, HashSet<u32>>, candidates: &HashSet<u32>) -> u32 {
    if let Some(set) = graph.get(&target) {
        if let Some(&child) = set.intersection(candidates).next() {
            return find_leaf(child, graph, candidates);
        }
    }
    target
}

fn part1(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    let before_rules = build_before_rules(rules);

    updates
        .iter()
        .filter_map(|pages| {
            has_correct_ordering(pages, &before_rules)
                .then_some(pages.as_slice().middle().unwrap().to_owned())
        })
        .sum::<u32>()
}

fn part2(rules: &[(u32, u32)], updates: &[Vec<u32>]) -> u32 {
    let before_rules = build_before_rules(rules);
    let after_rules = build_after_rules(rules);

    updates
        .iter()
        .filter(|pages| !has_correct_ordering(pages, &before_rules))
        .map(|pages| {
            let mut remaining = pages.clone().into_iter().collect::<HashSet<u32>>();
            let mut ordered = Vec::new();
            while let Some(&target) = remaining.iter().next() {
                let leaf = find_leaf(target, &after_rules, &remaining);
                ordered.push(leaf);
                remaining.remove(&leaf);
            }
            ordered.as_slice().middle().unwrap().to_owned()
        })
        .sum::<u32>()
}

fn main() {
    let file = File::open("day5/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok);
    let mut lines: Box<dyn Iterator<Item = String>> = Box::new(lines);
    let rules_loader = RulesLoader::new(&mut lines);
    let rules = rules_loader.collect::<Vec<_>>();
    let updates_loader = UpdatesLoader::new(&mut lines);
    let updates = updates_loader.collect::<Vec<_>>();

    println!("Part 1: {}", part1(&rules, &updates));
    println!("Part 2: {}", part2(&rules, &updates));
}
