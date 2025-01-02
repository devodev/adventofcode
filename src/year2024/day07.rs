use itertools::Itertools;

use anyhow::{Context, Result};
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

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

fn part1(input: String) -> Result<()> {
    let ops = [Operation::Add, Operation::Multiply];
    let result = compute(input, &ops)?;
    println!("{result}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let ops = [Operation::Add, Operation::Multiply, Operation::Concat];
    let result = compute(input, &ops)?;
    println!("{result}");
    Ok(())
}

fn compute(input: String, ops: &[Operation]) -> Result<u64> {
    let mut result = 0;
    for line in input.lines() {
        let Some((test_val, rem)) = line.split_once(':') else {
            anyhow::bail!("invalid input line format: must contain ':'");
        };
        let test_val = test_val.parse::<u64>().context("parsing test value as integer")?;
        let nums = {
            let nums: std::result::Result<Vec<u64>, std::num::ParseIntError> =
                rem.split_whitespace().map(str::parse::<u64>).collect();
            nums.context("parse equation numbers as integers")?
        };

        let ops_matrix: Vec<_> = itertools::repeat_n(ops, nums.len() - 1)
            .multi_cartesian_product()
            .collect();
        info!("ops_matrix:{ops_matrix:?} (vals:{nums:?})");
        for matrix in ops_matrix.iter() {
            let candidate: u64 =
                nums.iter()
                    .tuple_windows()
                    .enumerate()
                    .fold(0, |acc, (idx, (l, r))| match matrix[idx] {
                        Operation::Add => {
                            if acc == 0 {
                                acc + (l + r)
                            } else {
                                acc + r
                            }
                        }
                        Operation::Multiply => {
                            if acc == 0 {
                                acc + (l * r)
                            } else {
                                acc * r
                            }
                        }
                        Operation::Concat => {
                            let digits = (*r as f64).log10().floor() as u32 + 1;
                            if acc == 0 {
                                acc + ((l * (10_u64.pow(digits))) + r)
                            } else {
                                (acc * (10_u64.pow(digits))) + r
                            }
                        }
                    });
            if candidate == test_val {
                info!("test_val:{test_val} == candidate:{candidate}");
                result += candidate;
                break;
            }
            info!("test_val:{test_val} != candidate:{candidate}");
        }
    }

    Ok(result)
}
