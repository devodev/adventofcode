use std::collections::HashMap;

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
    let rule_pairs: Vec<_> = input
        .lines()
        .filter(|l| l.contains('|'))
        .map(|l| {
            let mut split = l.splitn(2, '|');
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    let updates: Vec<_> = input
        .lines()
        .filter(|l| l.contains(','))
        .map(|l| {
            l.trim()
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mapping_right_left: HashMap<u32, Vec<_>> = rule_pairs.iter().fold(HashMap::new(), |mut acc, p| {
        acc.entry(p.1).or_default().push(p.0);
        acc
    });
    let mapping_left_right: HashMap<u32, Vec<_>> = rule_pairs.iter().fold(HashMap::new(), |mut acc, p| {
        acc.entry(p.0).or_default().push(p.1);
        acc
    });

    let mut sum = 0;
    for up in updates.iter() {
        let mut valid = true;
        // compare pairs left to right
        for i in 0..up.len() - 1 {
            // compare pair ordering
            let (left, right) = (up[i], up[i + 1]);
            if mapping_left_right.get(&left).is_some_and(|vals| vals.contains(&right)) {
                // pair order is ok
                continue;
            }
            if mapping_right_left.get(&left).is_some_and(|vals| vals.contains(&right)) {
                // order is reversed
                valid = false;
                break;
            }
        }
        if valid {
            sum += up[up.len() / 2]
        }
    }
    println!("{sum}");

    Ok(())
}

fn part2(input: String) -> Result<()> {
    let rule_pairs: Vec<_> = input
        .lines()
        .filter(|l| l.contains('|'))
        .map(|l| {
            let mut split = l.splitn(2, '|');
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    let updates: Vec<_> = input
        .lines()
        .filter(|l| l.contains(','))
        .map(|l| {
            l.trim()
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mapping_right_left: HashMap<u32, Vec<_>> = rule_pairs.iter().fold(HashMap::new(), |mut acc, p| {
        acc.entry(p.1).or_default().push(p.0);
        acc
    });
    let mapping_left_right: HashMap<u32, Vec<_>> = rule_pairs.iter().fold(HashMap::new(), |mut acc, p| {
        acc.entry(p.0).or_default().push(p.1);
        acc
    });

    let mut sum = 0;
    let mut updates = updates;
    for up in updates.iter_mut() {
        let mut valid = true;
        loop {
            let mut swap_occured = false;
            // compare pairs left to right
            for i in 0..up.len() - 1 {
                // compare pair ordering
                let (left, right) = (up[i], up[i + 1]);
                if mapping_left_right.get(&left).is_some_and(|vals| vals.contains(&right)) {
                    // pair order is ok
                    continue;
                }
                if mapping_right_left.get(&left).is_some_and(|vals| vals.contains(&right)) {
                    // order is reversed, swap
                    (up[i], up[i + 1]) = (right, left);
                    swap_occured = true;
                }
            }
            if !swap_occured {
                // no swap in this iteration, update order is ok
                break;
            }
            valid = false;
        }
        if !valid {
            sum += up[up.len() / 2]
        }
    }
    println!("{sum}");

    Ok(())
}
