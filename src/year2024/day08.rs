use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

fn part1(input: String) -> Result<()> {
    let width = input.split_once('\n').unwrap().0.len().try_into().unwrap();
    let height = input.lines().count().try_into().unwrap();
    info!("width:{width} height:{height}");

    let antennas: HashMap<String, Vec<Position>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter(|(_, c)| *c != '.').map(move |(x, c)| {
                (
                    c.to_string(),
                    Position {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                )
            })
        })
        .fold(HashMap::new(), |mut acc, (s, p)| {
            acc.entry(s).or_default().push(p);
            acc
        });
    info!("antennas: {antennas:?}");

    let positions: HashSet<Position> = antennas.values().flatten().cloned().collect();
    info!("positions: {positions:?}");

    let mut antinodes = HashSet::new();
    for positions in antennas.values().filter(|p| p.len() > 1) {
        for i in 0..positions.len() {
            let left = &positions[i];
            for right in positions[0..i].iter().chain(&positions[i + 1..]) {
                info!("left:{left:?} right:{right:?}");
                let vector = (right.x - left.x, right.y - left.y);
                let new_pos = Position {
                    x: right.x + vector.0,
                    y: right.y + vector.1,
                };
                if new_pos.x < 0 || new_pos.x >= width {
                    continue;
                }
                if new_pos.y < 0 || new_pos.y >= height {
                    continue;
                }
                antinodes.insert(new_pos);
            }
        }
    }
    info!("antinodes: {antinodes:?}");

    println!("{}", antinodes.len());
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let width = input.split_once('\n').unwrap().0.len().try_into().unwrap();
    let height = input.lines().count().try_into().unwrap();
    info!("width:{width} height:{height}");

    let antennas: HashMap<String, Vec<Position>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter(|(_, c)| *c != '.').map(move |(x, c)| {
                (
                    c.to_string(),
                    Position {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                )
            })
        })
        .fold(HashMap::new(), |mut acc, (s, p)| {
            acc.entry(s).or_default().push(p);
            acc
        });
    info!("antennas: {antennas:?}");

    let positions: HashSet<Position> = antennas.values().flatten().cloned().collect();
    info!("positions: {positions:?}");

    let mut antinodes = HashSet::new();
    for positions in antennas.values().filter(|p| p.len() > 1) {
        for pos_i in 0..positions.len() {
            let left = &positions[pos_i];
            'positions: for right in positions[0..pos_i].iter().chain(&positions[pos_i + 1..]) {
                antinodes.insert(Position { x: left.x, y: left.y });
                antinodes.insert(Position { x: right.x, y: right.y });

                info!("left:{left:?} right:{right:?}");
                let vector = (right.x - left.x, right.y - left.y);
                for i in 1.. {
                    let new_pos = Position {
                        x: right.x + vector.0 * i,
                        y: right.y + vector.1 * i,
                    };
                    if new_pos.x < 0 || new_pos.x >= width {
                        continue 'positions;
                    }
                    if new_pos.y < 0 || new_pos.y >= height {
                        continue 'positions;
                    }
                    antinodes.insert(new_pos);
                }
            }
        }
    }
    info!("antinodes: {antinodes:?}");

    println!("{}", antinodes.len());
    Ok(())
}
