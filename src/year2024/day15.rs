use std::collections::VecDeque;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use tracing::{debug, info};

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
    info!("\n{map}");

    map.resolve();
    info!("\n{map}");

    let coordinates = map.gps_coordinates();
    let sum: usize = coordinates.iter().sum();
    println!("{sum}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let mut map = Map::from_str(&input).unwrap().make_wide();
    debug!("{map:?}");
    info!("\n{map}");

    map.resolve();
    info!("\n{map}");

    let coordinates = map.gps_coordinates();
    let sum: usize = coordinates.iter().sum();
    println!("{sum}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Standard;
#[derive(Debug, Clone)]
struct Wide;

#[derive(Debug, Clone)]
struct Map<State> {
    map: Vec<Vec<char>>,
    moves: VecDeque<char>,
    _state: PhantomData<State>,
}

impl FromStr for Map<Standard> {
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
        Ok(Self {
            map,
            moves,
            _state: PhantomData::<Standard>,
        })
    }
}

impl<State> Display for Map<State> {
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

impl<State> Map<State> {
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
}

impl Map<Standard> {
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
                match new_entry {
                    'O' => {
                        continue;
                    }
                    '.' => {
                        can_swap = true;
                        break;
                    }
                    _ => unreachable!(),
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

    fn make_wide(self) -> Map<Wide> {
        let wide_map = self
            .map
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|c| match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Map {
            map: wide_map,
            moves: self.moves,
            _state: PhantomData::<Wide>,
        }
    }
}

impl Map<Wide> {
    fn resolve(&mut self) {
        let mut pos = self.start_pos();
        while let Some(mov) = self.moves.pop_front() {
            debug!("\n{self}");
            let (x, y) = (pos.0 as isize, pos.1 as isize);

            let mut can_swap = true;
            let mut swaps = Vec::new();

            let mut stack = VecDeque::new();
            stack.push_front((x, y));

            while let Some((x, y)) = stack.pop_back() {
                let entry = self.get_map_entry((x, y));
                let (new_x, new_y) = new_pos((x, y), mov);
                let new_entry = self.get_map_entry((new_x, new_y));
                debug!("pos:({x}, {y}) ({entry}) new_pos:({new_x}, {new_y}) ({new_entry}) mov:{mov}");
                if new_entry == '#' {
                    can_swap = false;
                    break;
                }
                if !swaps.contains(&((x, y), (new_x, new_y))) {
                    debug!("adding swap: {:?}", ((x, y), (new_x, new_y)));
                    swaps.push(((x, y), (new_x, new_y)));
                }
                match (new_entry, mov) {
                    (']', '^' | 'v') => {
                        // v
                        // [|]
                        // ^
                        debug!("adding stack: {:?} {:?}", (new_x - 1, new_y), (new_x, new_y));
                        stack.push_front((new_x - 1, new_y));
                        stack.push_front((new_x, new_y));
                    }
                    ('[', '^' | 'v') => {
                        //   v
                        // [|]
                        //   ^
                        debug!("adding stack: {:?} {:?}", (new_x + 1, new_y), (new_x, new_y));
                        stack.push_front((new_x + 1, new_y));
                        stack.push_front((new_x, new_y));
                    }
                    ('[' | ']', '<' | '>') => {
                        stack.push_front((new_x, new_y));
                    }
                    ('.', '<' | '>') => {
                        break;
                    }
                    ('.', _) => {}
                    _ => unreachable!(),
                }
            }
            if can_swap {
                while let Some((left, right)) = swaps.pop() {
                    debug!("swapping ({left:?}) ({right:?})");
                    self.swap(left, right);
                }
                let new_pos = new_pos((pos.0 as isize, pos.1 as isize), mov);
                pos = (new_pos.0 as usize, new_pos.1 as usize);
            }
        }
    }

    // This warehouse also uses GPS to locate the boxes. For these larger boxes, distances are measured
    // from the edge of the map to the closest edge of the box in question. So, the box shown below has
    // a distance of 1 from the top edge of the map and 5 from the left edge of the map, resulting in a
    // GPS coordinate of 100 * 1 + 5 = 105.
    //
    //   ##########
    //   ##...[]...
    //   ##........
    fn gps_coordinates(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .tuple_windows()
                    .filter(|((_, c1), (_, c2))| *c1 == &'[' && *c2 == &']')
                    .map(move |((x1, _), _)| (y * 100) + x1)
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
