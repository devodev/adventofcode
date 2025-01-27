use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::{Deref, Rem};
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;
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
    let mut robots = Robots::from_str(&input).unwrap();
    info!("robots: {robots:?}");

    let iterations = 100;
    (0..iterations).for_each(|_| robots.move_once());
    info!("new_robots (i={iterations}): {robots:?}");

    let quadrants = robots.compute_quadrants();
    let (q1, q2, q3, q4) = quadrants.compute_sums();
    info!("q1:{q1}, q2:{q2} q3:{q3}, q4:{q4}");

    let safety_factor = q1 * q2 * q3 * q4;
    println!("{safety_factor}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let mut robots = Robots::from_str(&input).unwrap();
    info!("robots: {robots:?}");

    let max_iterations = 10_000;
    let mut list: Vec<_> = (0..max_iterations)
        .map(|i| {
            if i != 0 {
                robots.move_once();
            }
            // I got this idea from reddit.. still very satisfying!
            //
            // ###############################
            // #.............................#
            // #.............................#
            // #.............................#
            // #.............................#
            // #..............#..............#
            // #.............###.............#
            // #............#####............#
            // #...........#######...........#
            // #..........#########..........#
            // #............#####............#
            // #...........#######...........#
            // #..........#########..........#
            // #.........###########.........#
            // #........#############........#
            // #..........#########..........#
            // #.........###########.........#
            // #........#############........#
            // #.......###############.......#
            // #......#################......#
            // #........#############........#
            // #.......###############.......#
            // #......#################......#
            // #.....###################.....#
            // #....#####################....#
            // #.............###.............#
            // #.............###.............#
            // #.............###.............#
            // #.............................#
            // #.............................#
            // #.............................#
            // #.............................#
            // ###############################
            let neighboor_prob_score = robots.compute_neighboor_prob_score();
            (neighboor_prob_score, robots.clone(), i)
        })
        .collect();
    list.sort_by(|a, b| b.0.cmp(&a.0));

    let candidate = list.first().unwrap();
    info!("iteration {}:\n{}", candidate.0, candidate.1);

    println!("{}", candidate.2);
    Ok(())
}

#[derive(Debug, Clone)]
struct Robot {
    position: (u32, u32),
    velocity: (i32, i32),
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());
        let caps = RE.captures(s).unwrap();
        let (_, [px, py, vx, vy]) = caps.extract();

        Ok(Robot {
            position: (px.parse::<u32>().unwrap(), py.parse::<u32>().unwrap()),
            velocity: (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap()),
        })
    }
}

impl Robot {
    fn simulate_move(&self, width: u32, height: u32) -> (u32, u32) {
        let (width, height) = (width as i32, height as i32);
        (
            ((self.position.0 as i32 + self.velocity.0).rem(width) + width).rem(width) as u32,
            ((self.position.1 as i32 + self.velocity.1).rem(height) + height).rem(height) as u32,
        )
    }
}

#[derive(Debug, Clone)]
struct Robots {
    robots: Vec<Robot>,
    width: u32,
    height: u32,
}

impl FromStr for Robots {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let robots: Vec<_> = s.lines().flat_map(Robot::from_str).collect();
        let (width, height) = {
            let (width, height) = robots.iter().fold((0, 0), |mut acc, r| {
                if r.position.0 > acc.0 {
                    acc.0 = r.position.0;
                }
                if r.position.1 > acc.1 {
                    acc.1 = r.position.1;
                }
                acc
            });
            (width + 1, height + 1)
        };
        Ok(Self { robots, width, height })
    }
}

impl Deref for Robots {
    type Target = Vec<Robot>;

    fn deref(&self) -> &Self::Target {
        &self.robots
    }
}

impl Display for Robots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let positions: HashSet<(u32, u32)> = HashSet::from_iter(self.robots.iter().map(|r| r.position));
        for y in 0..self.height {
            for x in 0..self.width {
                if positions.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Quadrants(
    HashMap<(u32, u32), usize>,
    HashMap<(u32, u32), usize>,
    HashMap<(u32, u32), usize>,
    HashMap<(u32, u32), usize>,
);

impl Quadrants {
    fn compute_sums(&self) -> (usize, usize, usize, usize) {
        (
            self.0.values().sum(),
            self.1.values().sum(),
            self.2.values().sum(),
            self.3.values().sum(),
        )
    }
}

impl Robots {
    fn move_once(&mut self) {
        self.robots.iter_mut().for_each(|r| {
            r.position = r.simulate_move(self.width, self.height);
        });
    }

    fn compute_quadrants(&self) -> Quadrants {
        let is_even = (self.width.rem(2) == 0, self.height.rem(2) == 0);
        let center = (self.width / 2, self.height / 2);
        self.robots
            .iter()
            .filter(|r| (is_even.0 || r.position.0 != center.0) && (is_even.1 || r.position.1 != center.1))
            .fold(Quadrants::default(), |mut acc, r| {
                let (is_left, is_up) = (r.position.0 < center.0, r.position.1 < center.1);
                let q = match (is_left, is_up) {
                    (true, true) => &mut acc.0,
                    (true, false) => &mut acc.1,
                    (false, true) => &mut acc.2,
                    (false, false) => &mut acc.3,
                };
                q.entry(r.position).and_modify(|e| *e += 1).or_insert(1);
                acc
            })
    }

    fn compute_neighboor_prob_score(&self) -> u32 {
        let positions: HashSet<(u32, u32)> = HashSet::from_iter(self.robots.iter().map(|r| r.position));

        let directions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        self.robots
            .iter()
            .flat_map(|r| {
                directions.iter().map(|d| {
                    let pos = (r.position.0 as i32 + d.0, r.position.1 as i32 + d.1);
                    if pos.0 >= 0 && pos.1 >= 0 && positions.contains(&(pos.0 as u32, pos.1 as u32)) {
                        1
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}
