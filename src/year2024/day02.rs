use std::cmp::Ordering;

use anyhow::Result;

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
    let safe_count: u32 = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse::<u32>().expect("parsing level as int"))
                .collect::<Vec<_>>()
        })
        .filter_map(|levels| is_safe(&levels).then_some(1))
        .sum();
    println!("{safe_count}");

    Ok(())
}

fn part2(input: String) -> Result<()> {
    let safe_count: u32 = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse::<u32>().expect("parsing level as int"))
                .collect::<Vec<_>>()
        })
        .filter_map(|levels| {
            let mut safe = is_safe(&levels);
            if !safe {
                for i in 0..levels.len() {
                    let levels: Vec<_> = levels
                        .iter()
                        .enumerate()
                        .filter(|(n, _)| *n != i)
                        .map(|(_, v)| *v)
                        .collect();
                    safe = is_safe(&levels);
                    if safe {
                        break;
                    }
                }
            }
            safe.then_some(1)
        })
        .sum();
    println!("{safe_count}");

    Ok(())
}

// For a report to be safe, it must satisfy the following rules:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
fn is_safe(levels: &[u32]) -> bool {
    let mut ordering = None;
    for (left, right) in levels.iter().zip(levels.iter().skip(1)) {
        match (left.cmp(right), ordering) {
            (Ordering::Equal, _) => return false,
            (new_ordering, Some(ord)) if ord != new_ordering => return false,
            (new_ordering, current_ordering) => match left.abs_diff(*right) {
                1..=3 => {
                    if current_ordering.is_none() {
                        ordering = Some(new_ordering);
                    }
                }
                _ => return false,
            },
        }
    }
    true
}
