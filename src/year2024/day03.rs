use std::ops::Range;

use anyhow::Result;
use regex::Regex;

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
    let sum = mul_sum(&input);
    println!("{sum}");

    Ok(())
}

fn part2(input: String) -> Result<()> {
    let ranges = find_ranges(&input);
    let input = trim_input(input.clone(), ranges);
    let sum = mul_sum(&input);
    println!("{sum}");

    Ok(())
}

fn mul_sum(input: &str) -> u64 {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re_mul
        .captures_iter(input)
        .map(|caps| {
            let (_, [left, right]) = caps.extract();
            left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap()
        })
        .sum()
}

fn trim_input(input: String, ranges: Vec<Range<usize>>) -> String {
    let mut input = input;
    let mut offset = 0;
    for r in ranges {
        let (start, end) = (r.start - offset, r.end - offset);
        input.replace_range(start..end, "");
        offset += r.len()
    }
    input
}

fn find_ranges(input: &str) -> Vec<Range<usize>> {
    let re_dontdo = Regex::new(r"don't\(\)|do\(\)").unwrap();
    let mut ranges: Vec<Range<usize>> = Vec::new();
    let mut current_start = None;
    for m in re_dontdo.find_iter(input) {
        match m.len() {
            // dont
            7 => _ = current_start.get_or_insert(m.start()),
            // do()
            4 => {
                if let Some(start) = current_start.take() {
                    ranges.push(start..m.end());
                }
            }
            _ => {}
        }
    }
    if let Some(start) = current_start.take() {
        ranges.push(start..input.len());
    }
    ranges
}
