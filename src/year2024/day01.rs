use std::collections::BinaryHeap;

use adventofcode::split2;
use anyhow::{anyhow, Result};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Part 1 (binary heap variant).
    ///
    /// Uses a BinaryHeap to accumulate column values and have them sorted on insert.
    Part1BinaryHeap,

    /// Part 1 (sort variant).
    ///
    /// Uses a Vec to accumulate column values and sorts them after insert.
    Part1Sort,

    /// Part 2.
    Part2,
}

impl Args {
    pub fn run(self, input: String) -> Result<()> {
        match self.command {
            Commands::Part1BinaryHeap => part1_binary_heap(input),
            Commands::Part1Sort => part1_sort(input),
            Commands::Part2 => part2(input),
        }
    }
}

fn part1_binary_heap(input: String) -> Result<()> {
    let mut col1 = BinaryHeap::new();
    let mut col2 = BinaryHeap::new();
    for line in input.lines() {
        let (left, right) = split2::<u32>(line)?;
        col1.push(left);
        col2.push(right);
    }

    let col1 = col1.into_sorted_vec();
    let col2 = col2.into_sorted_vec();

    let total_distance: u32 = col1.iter().zip(col2).map(|(left, right)| left.abs_diff(right)).sum();
    println!("{total_distance}");

    Ok(())
}

fn part1_sort(input: String) -> Result<()> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in input.lines() {
        let (left, right) = split2::<u32>(line)?;
        col1.push(left);
        col2.push(right);
    }

    col1.sort();
    col2.sort();

    let total_distance: u32 = col1.iter().zip(col2).map(|(left, right)| left.abs_diff(right)).sum();
    println!("{total_distance}");

    Ok(())
}

fn part2(_input: String) -> Result<()> {
    Err(anyhow!("not implemented"))
}
