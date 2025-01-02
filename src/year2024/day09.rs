use std::ops::{Range, Rem};

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
    let diskmap: Vec<String> = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let is_disk = i.rem(2) == 0;
            let element = if is_disk {
                (i / 2).to_string()
            } else {
                String::from(".")
            };
            let n = c.to_digit(10).unwrap();
            itertools::repeat_n(element, n as usize)
        })
        .collect();
    info!("diskmap: {} (len:{})", diskmap.join(""), diskmap.len());

    let disk_candidates: Vec<_> = diskmap
        .iter()
        .enumerate()
        .filter(|(_, c)| *c != ".")
        .map(|(i, _)| i)
        .rev()
        .collect();
    info!("disk_candidates: {disk_candidates:?} (len:{})", disk_candidates.len());

    let swaps: Vec<_> = diskmap
        .iter()
        .enumerate()
        .filter(|(_, c)| *c == ".")
        .filter(|(i, _)| *i < disk_candidates.len())
        .enumerate()
        .map(|(dot_i, (i, _))| (i, disk_candidates[dot_i]))
        .collect();
    info!("swaps: {swaps:?}");

    let mut diskmap = diskmap;
    for swap in swaps {
        diskmap.swap(swap.0, swap.1);
    }
    info!("new diskmap: {}", diskmap.join(""));

    let cheksum = diskmap
        .iter()
        .enumerate()
        .filter(|(_, c)| *c != ".")
        .fold(0, |acc, (i, c)| acc + (i * c.parse::<usize>().unwrap()));

    println!("{cheksum}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let mut diskmap: Vec<String> = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let is_disk = i.rem(2) == 0;
            let element = if is_disk {
                (i / 2).to_string()
            } else {
                String::from(".")
            };
            let n = c.to_digit(10).unwrap();
            itertools::repeat_n(element, n as usize)
        })
        .collect();
    info!("diskmap: {} (len:{})", diskmap.join(""), diskmap.len());

    let (file_ranges, _) = compute_ranges(&diskmap);
    for f_range in file_ranges.iter().rev().cloned() {
        let f_start = f_range.start;
        let f_size = f_range.len();
        let (_, empty_ranges) = compute_ranges(&diskmap);
        for e_range in empty_ranges {
            if e_range.end > f_start {
                break;
            }
            let e_size = e_range.len();
            if e_size < f_size {
                continue;
            }
            for (left, right) in e_range.zip(f_range.clone()) {
                diskmap.swap(left, right);
            }
        }
    }
    info!("diskmap: {} (len:{})", diskmap.join(""), diskmap.len());

    let cheksum = diskmap
        .iter()
        .enumerate()
        .filter(|(_, c)| *c != ".")
        .fold(0, |acc, (i, c)| acc + (i * c.parse::<usize>().unwrap()));

    println!("{cheksum}");
    Ok(())
}

fn compute_ranges(diskmap: &[String]) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut file_ranges = Vec::new();
    let mut empty_ranges = Vec::new();
    let mut start = 0;
    let mut current_c = None;
    for (i, c) in diskmap.iter().enumerate() {
        let Some(cc) = &current_c else {
            current_c = Some(c.to_string());
            continue;
        };
        if cc == c {
            continue;
        }

        if cc == "." {
            empty_ranges.push(start..i);
        } else {
            file_ranges.push(start..i);
        }
        start = i;
        current_c = Some(c.to_string());
    }
    if let Some(cc) = &current_c {
        if cc == "." {
            empty_ranges.push(start..diskmap.len());
        } else {
            file_ranges.push(start..diskmap.len());
        }
    }
    info!("file_ranges:{file_ranges:?}");
    info!("empty_ranges:{empty_ranges:?}");

    (file_ranges, empty_ranges)
}
