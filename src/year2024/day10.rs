use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use tracing::info;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Part 1.
    Part1,

    /// Part 2.
    Part2,
}

impl Args {
    pub fn run(self, input: String) -> Result<()> {
        match self.command {
            Commands::Part1 => part1(input),
            Commands::Part2 => part2(input),
        }
    }
}

fn part1(input: String) -> Result<()> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    info!("grid: {grid:?}");

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;
    let trailheads: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect();
    info!("trailheads: {trailheads:?}");

    let mut scores = Vec::new();
    for head in trailheads.iter() {
        let mut tails = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(((head.0 as isize, head.1 as isize), 1));

        while let Some((pos, val)) = queue.pop_front() {
            let directions = [
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
            ];
            for (x, y) in directions {
                if x < 0 || x >= width {
                    continue;
                }
                if y < 0 || y >= height {
                    continue;
                }
                if grid[y as usize][x as usize] != val {
                    continue;
                }
                if val == 9 {
                    tails.insert((x, y));
                    continue;
                }
                queue.push_front(((x, y), val + 1));
            }
        }
        scores.push(tails.len());
    }
    info!("scores: {scores:?}");

    let sum: usize = scores.iter().sum();
    println!("{sum}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    info!("grid: {grid:?}");

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;
    let trailheads: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect();
    info!("trailheads: {trailheads:?}");

    let mut ratings = Vec::new();
    for head in trailheads.iter() {
        let mut tails = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back(((head.0 as isize, head.1 as isize), 1));

        while let Some((pos, val)) = queue.pop_front() {
            let directions = [
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
            ];
            for (x, y) in directions {
                if x < 0 || x >= width {
                    continue;
                }
                if y < 0 || y >= height {
                    continue;
                }
                if grid[y as usize][x as usize] != val {
                    continue;
                }
                if val == 9 {
                    tails.entry((x, y)).and_modify(|v: &mut usize| *v += 1).or_insert(1);
                    continue;
                }
                queue.push_front(((x, y), val + 1));
            }
        }
        ratings.push(tails.values().sum());
    }
    info!("rating: {ratings:?}");

    let sum: usize = ratings.iter().sum();
    println!("{sum}");
    Ok(())
}
