use std::{collections::HashMap, ops::Rem};

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
    let stones = compute_arrangement(&input, 25);

    println!("{stones}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let stones = compute_arrangement_precompute(&input, 75, 1..=10, 40);

    println!("{stones}");
    Ok(())
}

fn compute_arrangement(input: &str, blinks: usize) -> usize {
    let mut stones: Vec<u128> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    for i in 0..blinks {
        info!("blink:{i}");
        let mut new_arrangement = Vec::new();
        for stone in stones.iter() {
            new_arrangement.extend(core(*stone));
        }
        stones = new_arrangement;
        info!("blink:{i} (stones:{})", stones.len());
    }
    stones.len()
}

fn compute_arrangement_precompute(
    input: &str,
    blinks: usize,
    pc_range: impl IntoIterator<Item = u128>,
    pc_blinks: usize,
) -> usize {
    let mut stones: Vec<u128> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let precomputed: HashMap<u128, Vec<u128>> = pc_range
        .into_iter()
        .map(|val| {
            let mut values = Vec::new();
            let mut stones = vec![val];
            for i in 0..pc_blinks {
                info!("precomputed {val} blink:{i}");
                let mut new_arrangement = Vec::new();
                for stone in stones.iter() {
                    new_arrangement.extend(core(*stone));
                }
                values.push(new_arrangement.len() as u128);
                stones = new_arrangement;
                info!("blink:{i} (stones:{})", stones.len());
            }
            (val, values)
        })
        .collect();
    info!("precomputed: {precomputed:?}");

    let mut precomputed_count = 0;
    for i in 0..blinks {
        info!("blink:{i}");
        let mut new_arrangement = Vec::new();
        for stone in stones.iter() {
            if let Some(pc) = precomputed.get(stone) {
                if let Some(pc_val) = pc.get(blinks - i - 1) {
                    precomputed_count += pc_val;
                    continue;
                }
            }
            new_arrangement.extend(core(*stone));
        }
        stones = new_arrangement;
        info!("blink:{i} (stones:{}) (precomputed:{precomputed_count})", stones.len());
    }
    stones.len() + precomputed_count as usize
}

fn core(stone: u128) -> Vec<u128> {
    // If the stone is engraved with the number 0, it is replaced
    // by a stone engraved with the number 1.
    if stone == 0 {
        return vec![1];
    }
    // If the stone is engraved with a number that has an even number
    // of digits, it is replaced by two stones. The left half of the
    // digits are engraved on the new left stone, and the right half
    // of the digits are engraved on the new right stone.
    // (The new numbers don't keep extra leading zeroes: 1000 would
    // become stones 10 and 0.)
    let digits = (stone as f64).log10().floor() as u32 + 1;
    if digits.rem(2) == 0 {
        // When you deal with representations of numbers in base B, the trick to removing
        // leading digits is using remainders of division by powers of the base B; to remove
        // trailing digits, use integer division by powers of the base.
        // - Specifically, to remove k leading decimal digits of an n-digit decimal number X,
        //   obtain the remainder of the division of X by ten to the power of n-k.
        // - To remove the trailing k decimal digits, use integer division by 10 ^ 3
        // https://stackoverflow.com/a/20828756
        return vec![stone / 10_u128.pow(digits / 2), stone.rem(10_u128.pow(digits / 2))];
    }
    // If none of the other rules apply, the stone is replaced by a new stone;
    // the old stone's number multiplied by 2024 is engraved on the new stone.
    vec![stone * 2024]
}
