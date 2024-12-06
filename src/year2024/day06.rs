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

#[derive(Debug, Clone, PartialEq)]
enum Position {
    Empty,
    Obstruction,
    Visited(Direction),
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn look_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    dir: Direction,
    pos: (usize, usize),
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Position>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: String) -> (Map, Guard) {
        let mut guard = None;
        let grid: Vec<Vec<Position>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Position::Empty,
                        '#' => Position::Obstruction,
                        '^' | 'v' | '<' | '>' => {
                            let dir = match c {
                                '^' => Direction::Up,
                                'v' => Direction::Down,
                                '<' => Direction::Left,
                                '>' => Direction::Right,
                                _ => unreachable!(),
                            };
                            let g = Guard {
                                dir: dir.clone(),
                                pos: (x, y),
                            };
                            if guard.is_some() {
                                panic!("there should be only one guard on the map (first:{guard:?}, second: {g:?})")
                            }
                            guard = Some(g);
                            Position::Visited(dir)
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let width = grid[0].len();
        let height = grid.len();
        let map = Map { grid, width, height };
        let guard = guard.expect("map should contain exactly one guard");

        (map, guard)
    }

    fn lookup_visited(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let mut pos = (pos.0 as isize, pos.1 as isize);
        loop {
            let new_pos = match dir {
                Direction::Up => (pos.0, pos.1 - 1),
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0 - 1, pos.1),
                Direction::Right => (pos.0 + 1, pos.1),
            };
            info!("lookup_visited: {pos:?} -> {new_pos:?}");
            // check if out of bounds
            if new_pos.0 < 0 || new_pos.0 as usize >= self.width || new_pos.1 < 0 || new_pos.1 as usize >= self.height {
                info!("out of bounds");
                return None;
            }
            pos = new_pos;
            // move to new pos
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            match &self.grid[new_pos.1][new_pos.0] {
                Position::Empty => continue,
                Position::Obstruction => return None,
                Position::Visited(v_dir) => {
                    if v_dir == &dir {
                        // try to check if obstruction in front as this is
                        // our indicator that it should loop
                        let front_pos = match dir {
                            Direction::Up => (pos.0, pos.1 - 1),
                            Direction::Down => (pos.0, pos.1 + 1),
                            Direction::Left => (pos.0 - 1, pos.1),
                            Direction::Right => (pos.0 + 1, pos.1),
                        };
                        if let Position::Obstruction = &self.grid[front_pos.1 as usize][front_pos.0 as usize] {
                            // check if obstruction in front
                            return Some(new_pos);
                        }
                        // return Some(new_pos);
                    }
                    // try ignoring different directions as this
                    // might be before turning right.
                    // alternatively, we could check if there is
                    // an obstruction in front instead.
                    //return None;
                    continue;
                }
            }
        }
    }

    fn print(&self, f: &dyn Fn((usize, usize)) -> bool) {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, pos) in line.iter().enumerate() {
                if f((x, y)) {
                    continue;
                }
                let c = match pos {
                    Position::Empty => '.',
                    Position::Obstruction => '#',
                    Position::Visited(direction) => match direction {
                        Direction::Up => 'i',
                        Direction::Down => '!',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    },
                };
                print!("{c}");
            }
            println!();
        }
    }

    fn guard_iter(&mut self, guard: &Guard) -> GuardIter {
        GuardIter {
            map: self,
            guard: guard.clone(),
            is_orignal_position: true,
        }
    }
}

struct GuardIter<'a> {
    map: &'a mut Map,
    guard: Guard,
    is_orignal_position: bool,
}

struct GuardWalkInsight {
    new_visit: bool,
    found_obs_causing_loop: Option<(usize, usize)>,
}

impl Iterator for GuardIter<'_> {
    type Item = GuardWalkInsight;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_pos = self.guard.pos;
        // account for original position and return guard pos right away
        if self.is_orignal_position {
            self.is_orignal_position = false;
            return Some(GuardWalkInsight {
                new_visit: true,
                found_obs_causing_loop: None,
            });
        }
        let new_pos = match self.guard.dir {
            Direction::Up => (cur_pos.0 as isize, cur_pos.1 as isize - 1),
            Direction::Down => (cur_pos.0 as isize, cur_pos.1 as isize + 1),
            Direction::Left => (cur_pos.0 as isize - 1, cur_pos.1 as isize),
            Direction::Right => (cur_pos.0 as isize + 1, cur_pos.1 as isize),
        };
        info!("{cur_pos:?} -> {new_pos:?}");
        // check if out of bounds
        if new_pos.0 < 0
            || new_pos.0 as usize >= self.map.width
            || new_pos.1 < 0
            || new_pos.1 as usize >= self.map.height
        {
            info!("out of bounds");
            return None;
        }
        // move to new pos
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        let grid = &mut self.map.grid;
        let (guard, new_visit) = match grid[new_pos.1][new_pos.0] {
            // if empty, set to visited
            Position::Empty => {
                grid[new_pos.1][new_pos.0] = Position::Visited(self.guard.dir.clone());
                let guard = Guard {
                    dir: self.guard.dir.clone(),
                    pos: new_pos,
                };
                (guard, true)
            }
            // if obstacle found, turn 90 degrees to the right
            Position::Obstruction => {
                let new_dir = self.guard.dir.look_right();
                let guard = match self.guard.dir {
                    Direction::Up => Guard {
                        dir: new_dir,
                        pos: (cur_pos.0 + 1, cur_pos.1),
                    },
                    Direction::Down => Guard {
                        dir: new_dir,
                        pos: (cur_pos.0 - 1, cur_pos.1),
                    },
                    Direction::Left => Guard {
                        dir: new_dir,
                        pos: (cur_pos.0, cur_pos.1 - 1),
                    },
                    Direction::Right => Guard {
                        dir: new_dir,
                        pos: (cur_pos.0, cur_pos.1 + 1),
                    },
                };
                let new_visit = grid[guard.pos.1][guard.pos.0] == Position::Empty;
                grid[guard.pos.1][guard.pos.0] = Position::Visited(guard.dir.clone());
                (guard, new_visit)
            }
            // if visited, do nothing
            Position::Visited(_) => {
                let guard = Guard {
                    dir: self.guard.dir.clone(),
                    pos: new_pos,
                };

                (guard, false)
            }
        };
        // search whether adding an obstruction in front of our pos
        // would result in a loop. If we search from our current pos
        // to our right and find a visited pos in the same direction,
        // we can (probably?) assume that placing an obstruction in
        // front would result in a loop.
        let found_obs_causing_loop = self.map.lookup_visited(guard.pos, guard.dir.look_right());
        // map to show the position of the obstruction instead of the found visited pos
        let found_obs_causing_loop = found_obs_causing_loop.map(|_| match guard.dir {
            Direction::Up => (guard.pos.0, guard.pos.1 - 1),
            Direction::Down => (guard.pos.0, guard.pos.1 + 1),
            Direction::Left => (guard.pos.0 - 1, guard.pos.1),
            Direction::Right => (guard.pos.0 + 1, guard.pos.1),
        });
        info!("new guard: {guard:?}");
        self.guard = guard.clone();
        Some(GuardWalkInsight {
            new_visit,
            found_obs_causing_loop,
        })
    }
}

fn part1(input: String) -> Result<()> {
    let (map, guard) = Map::new(input);
    info!("map: {map:?}");
    info!("guard: {guard:?}");

    let mut map = map;
    let unique_visits = map.guard_iter(&guard).filter(|v| v.new_visit).count();
    info!("map: {map:?}");

    println!("{}", unique_visits);

    Ok(())
}

fn part2(input: String) -> Result<()> {
    let (map, guard) = Map::new(input);
    info!("map: {map:?}");
    info!("guard: {guard:?}");

    let mut map = map;
    let obs_causing_loop_count: Vec<_> = map
        .guard_iter(&guard)
        .filter_map(|v| v.found_obs_causing_loop)
        .inspect(|v| info!("found_obs_causing_loop: {v:?}"))
        .collect();
    println!("{}", obs_causing_loop_count.len());

    // print the map
    let print_fn = |pos| {
        for obs in obs_causing_loop_count.iter() {
            if obs == &pos {
                print!("O");
                return true;
            }
        }
        if guard.pos == pos {
            print!("@");
            true
        } else {
            false
        }
    };
    map.print(&print_fn);

    Ok(())
}
