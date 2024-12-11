use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::ControlFlow,
    time::Instant,
};

use anyhow::{anyhow, bail};
use ndarray::Array2;

#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    fn build(c: char) -> anyhow::Result<Self> {
        let d = match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => {
                bail!("Character '{c}' is not a valid direction");
            }
        };
        Ok(d)
    }
}

enum EndCondition {
    Cycle,
    OffGrid,
}

#[derive(Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    pub row: usize,
    pub col: usize,
    pub dir: Direction,
}

impl Position {
    fn new(row: usize, col: usize, dir: Direction) -> Self {
        Self { row, col, dir }
    }
}

#[derive(Default, Clone, Copy)]
struct Step {
    next_position: Position,
    has_obstacle: bool,
}

impl Step {
    fn new(next_position: Position, has_obstacle: bool) -> Self {
        Self {
            next_position,
            has_obstacle,
        }
    }
}

struct Map {
    grid: Array2<char>,
    step_north_grid: Array2<Step>,
    step_east_grid: Array2<Step>,
    step_south_grid: Array2<Step>,
    step_west_grid: Array2<Step>,
    start_position: Position,
    current_position: Position,
    steps_log: HashSet<Position>,
    record_path: bool,
}

impl Map {
    fn init_steps_north(grid: &Array2<char>, steps: &mut Array2<Step>, col: usize) {
        let (num_rows, _) = grid.dim();
        let mut next_position = Position::new(0, col, Direction::East);
        let mut has_obstacle = false;
        for row in 0..num_rows {
            if grid[[row, col]] == '#' {
                next_position = Position::new(row + 1, col, Direction::East);
                has_obstacle = true;
            }
            steps[[row, col]] = Step::new(next_position, has_obstacle);
        }
    }

    fn init_steps_east(grid: &Array2<char>, steps: &mut Array2<Step>, row: usize) {
        let (_, num_cols) = grid.dim();
        let mut next_position = Position::new(row, num_cols - 1, Direction::South);
        let mut has_obstacle = false;
        for col in (0..num_cols).rev() {
            if grid[[row, col]] == '#' {
                next_position = Position::new(row, col - 1, Direction::South);
                has_obstacle = true;
            }
            steps[[row, col]] = Step::new(next_position, has_obstacle);
        }
    }

    fn init_steps_south(grid: &Array2<char>, steps: &mut Array2<Step>, col: usize) {
        let (num_rows, _) = grid.dim();
        let mut next_position = Position::new(num_rows - 1, col, Direction::West);
        let mut has_obstacle = false;
        for row in (0..num_rows).rev() {
            if grid[[row, col]] == '#' {
                next_position = Position::new(row - 1, col, Direction::West);
                has_obstacle = true;
            }
            steps[[row, col]] = Step::new(next_position, has_obstacle);
        }
    }

    fn init_steps_west(grid: &Array2<char>, steps: &mut Array2<Step>, row: usize) {
        let (_, num_cols) = grid.dim();
        let mut next_position = Position::new(row, 0, Direction::North);
        let mut has_obstacle = false;
        for col in 0..num_cols {
            if grid[[row, col]] == '#' {
                next_position = Position::new(row, col + 1, Direction::North);
                has_obstacle = true;
            }
            steps[[row, col]] = Step::new(next_position, has_obstacle);
        }
    }

    fn build(mut grid: Array2<char>) -> anyhow::Result<Self> {
        let (num_rows, num_cols) = grid.dim();

        let start_position @ Position { row, col, .. } = (0..num_rows)
            .filter_map(|row| {
                (0..num_cols)
                    .filter_map(|col| {
                        Direction::build(grid[[row, col]])
                            .ok()
                            .map(|dir| Position::new(row, col, dir))
                    })
                    .next()
            })
            .next()
            .ok_or(anyhow!("No direction found on grid"))?;

        grid[[row, col]] = '.';

        let mut step_north_grid = Array2::<Step>::from_shape_vec(
            (num_rows, num_cols),
            vec![Step::default(); num_rows * num_cols],
        )
        .unwrap();
        let mut step_east_grid = step_north_grid.clone();
        let mut step_south_grid = step_north_grid.clone();
        let mut step_west_grid = step_north_grid.clone();
        for row in 0..num_rows {
            Self::init_steps_east(&grid, &mut step_east_grid, row);
            Self::init_steps_west(&grid, &mut step_west_grid, row);
        }
        for col in 0..num_cols {
            Self::init_steps_north(&grid, &mut step_north_grid, col);
            Self::init_steps_south(&grid, &mut step_south_grid, col);
        }

        let result = Self {
            grid,
            step_north_grid,
            step_east_grid,
            step_south_grid,
            step_west_grid,
            start_position,
            current_position: start_position,
            steps_log: HashSet::new(),
            record_path: true,
        };

        Ok(result)
    }

    fn get_range(&self, next: Position) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let Position { row, col, dir } = self.current_position;
        let Position {
            row: next_row,
            col: next_col,
            dir: next_dir,
        } = next;
        debug_assert_eq!(dir, next_dir);
        let from_row = move |r| (r, col);
        let from_col = move |c| (row, c);
        match dir {
            Direction::North => {
                debug_assert_eq!(col, next_col);
                Box::new((next_row..=row).rev().map(from_row))
            }
            Direction::East => {
                debug_assert_eq!(row, next_row);
                Box::new((col..=next_col).map(from_col))
            }
            Direction::South => {
                debug_assert_eq!(col, next_col);
                Box::new((row..=next_row).map(from_row))
            }
            Direction::West => {
                debug_assert_eq!(row, next_row);
                Box::new((next_col..=col).rev().map(from_col))
            }
        }
    }

    fn update(&mut self) -> ControlFlow<EndCondition, ()> {
        let Position { row, col, dir } = self.current_position;

        let Step {
            next_position,
            has_obstacle,
        } = match dir {
            Direction::North => self.step_north_grid[[row, col]],
            Direction::East => self.step_east_grid[[row, col]],
            Direction::South => self.step_south_grid[[row, col]],
            Direction::West => self.step_west_grid[[row, col]],
        };

        if self.record_path {
            for (r, c) in self.get_range(next_position) {
                self.grid[[r, c]] = '_';
            }
        }

        if self.steps_log.contains(&next_position) {
            return ControlFlow::Break(EndCondition::Cycle);
        }

        self.steps_log.insert(self.current_position);
        self.current_position = next_position;

        if has_obstacle {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(EndCondition::OffGrid)
        }
    }

    fn reset(&mut self) {
        self.current_position = self.start_position;
        self.steps_log.clear();
    }

    fn update_step_grids(&mut self, row: usize, col: usize) {
        Self::init_steps_east(&self.grid, &mut self.step_east_grid, row);
        Self::init_steps_west(&self.grid, &mut self.step_west_grid, row);
        Self::init_steps_north(&self.grid, &mut self.step_north_grid, col);
        Self::init_steps_south(&self.grid, &mut self.step_south_grid, col);
    }

    fn add_obstacle(&mut self, row: usize, col: usize) {
        debug_assert_ne!(self.grid[[row, col]], '#');
        self.grid[[row, col]] = '#';
        self.update_step_grids(row, col);
    }

    fn remove_obstacle(&mut self, row: usize, col: usize) {
        debug_assert_eq!(self.grid[[row, col]], '#');
        self.grid[[row, col]] = '.';
        self.update_step_grids(row, col);
    }
}

fn part1(grid: &Array2<char>) -> usize {
    let mut map = Map::build(grid.clone()).unwrap();
    while map.update().is_continue() {}
    let Map { grid, .. } = map;
    grid.into_iter().filter(|&c| c == '_').count()
}

fn part2(grid: &Array2<char>) -> usize {
    let (num_rows, num_cols) = grid.dim();
    let mut map = Map::build(grid.clone()).unwrap();
    let (start_row, start_col) = (map.current_position.row, map.current_position.col);
    while map.update().is_continue() {}

    let default_visited = (0..num_rows)
        .flat_map(|r| {
            (0..num_cols)
                .filter_map(|c| {
                    ((r, c) != (start_row, start_col) && map.grid[[r, c]] == '_').then_some((r, c))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map.reset();
    map.record_path = false;

    default_visited
        .into_iter()
        .filter(|&(r, c)| {
            map.add_obstacle(r, c);
            let end_condition = loop {
                if let ControlFlow::Break(b) = map.update() {
                    break b;
                }
            };
            map.reset();
            map.remove_obstacle(r, c);
            matches!(end_condition, EndCondition::Cycle)
        })
        .count()
}

fn main() {
    let file = File::open("day6/data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let rows = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let num_rows = rows.len();
    let num_cols = rows.first().unwrap().len();
    let vec = rows.concat().chars().collect::<Vec<_>>();
    let grid = Array2::<char>::from_shape_vec((num_rows, num_cols), vec).unwrap();

    println!("Part 1: {}", part1(&grid));
    let start = Instant::now();
    println!("Part 2: {}", part2(&grid));
    println!("Part 2 took {:?}", start.elapsed());
}
