use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;

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
    let mut map = Map::from_str(&input).unwrap();
    info!("{map:?}");
    info!("{map}");

    map.resolve();
    info!("{map}");

    let coordinates = map.gps_coordinates();
    let sum: usize = coordinates.iter().sum();
    println!("{sum}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let mut map = Map::from_str(&input).unwrap();
    info!("{map:?}");
    info!("{map}");

    map.resolve();
    info!("{map}");

    let coordinates = map.gps_coordinates();
    let sum: usize = coordinates.iter().sum();
    println!("{sum}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    moves: VecDeque<char>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (map, moves) =
            s.lines()
                .filter(|l| !l.trim().is_empty())
                .fold((Vec::new(), VecDeque::new()), |mut acc, l| {
                    if l.starts_with("#") {
                        acc.0.push(l.chars().collect::<Vec<_>>());
                    } else {
                        acc.1.extend(l.chars());
                    }
                    acc
                });
        Ok(Self { map, moves })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for c in row.iter() {
                write!(f, "{c}").unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl Map {
    fn resolve(&mut self) {
        let mut pos = self.start_pos();
        while let Some(mov) = self.moves.pop_front() {
            let (mut x, mut y) = (pos.0 as isize, pos.1 as isize);

            let mut can_swap = false;
            let mut stack = Vec::new();
            loop {
                let entry = self.get_map_entry((x, y));
                let (new_x, new_y) = new_pos((x, y), mov);
                let new_entry = self.get_map_entry((new_x, new_y));
                info!("pos:({x}, {y}) ({entry}) new_pos:({new_x}, {new_y}) ({new_entry}) mov:{mov}");
                if new_entry == '#' {
                    break;
                }
                stack.push(((x, y), (new_x, new_y)));
                (x, y) = (new_x, new_y);
                if new_entry == 'O' {
                    continue;
                }
                if new_entry == '.' {
                    can_swap = true;
                    break;
                }
            }
            if can_swap {
                while let Some((left, right)) = stack.pop() {
                    self.swap(left, right);
                }
                let new_pos = new_pos((pos.0 as isize, pos.1 as isize), mov);
                pos = (new_pos.0 as usize, new_pos.1 as usize);
            }
        }
    }

    fn start_pos(&self) -> (usize, usize) {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)))
            .find(|(_, _, c)| *c == &'@')
            .map(|(x, y, _)| (x, y))
            .unwrap()
    }

    fn get_map_entry(&self, (x, y): (isize, isize)) -> char {
        self.map[y as usize][x as usize]
    }

    fn swap(&mut self, (x, y): (isize, isize), (new_x, new_y): (isize, isize)) {
        let src_val = std::mem::replace(&mut self.map[y as usize][x as usize], ' ');
        self.map[y as usize][x as usize] = std::mem::replace(&mut self.map[new_y as usize][new_x as usize], src_val);
    }

    fn gps_coordinates(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| *c == &'O')
                    .map(move |(x, _)| (y * 100) + x)
            })
            .collect()
    }
}

fn new_pos((x, y): (isize, isize), mov: char) -> (isize, isize) {
    match mov {
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        _ => unreachable!(),
    }
}
