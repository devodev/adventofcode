use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

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
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    info!("grid: {grid:?}");

    let width = grid[0].len();
    let height = grid.len();

    let mut visited: Vec<Vec<bool>> = grid.iter().map(|r| r.iter().map(|_| false).collect()).collect();
    let mut region_mapping: HashMap<char, Vec<HashSet<(isize, isize)>>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            let c = grid[y][x];

            let mut region = HashSet::new();

            let mut queue = VecDeque::new();
            queue.push_back((x as isize, y as isize));

            while let Some((x, y)) = queue.pop_front() {
                region.insert((x, y));

                let directions = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                for (x, y) in directions {
                    if x < 0 || x >= width as isize {
                        continue;
                    }
                    if y < 0 || y >= height as isize {
                        continue;
                    }
                    if visited[y as usize][x as usize] {
                        continue;
                    }
                    if grid[y as usize][x as usize] != c {
                        continue;
                    }
                    visited[y as usize][x as usize] = true;
                    queue.push_front((x, y));
                }
            }
            info!("new region for '{c}': {region:?}");
            region_mapping
                .entry(c)
                .and_modify(|r| r.push(region.clone()))
                .or_insert(vec![region]);
        }
    }
    info!("visited: {visited:?}");
    info!("regions: {region_mapping:?}");

    let mut region_dimensions = Vec::new();
    for (c, regions) in region_mapping {
        for (i, region) in regions.iter().enumerate() {
            let area = region.len();
            let mut perimeter = 0;
            for (x, y) in region.clone() {
                let directions = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                for (x, y) in directions {
                    if region.contains(&(x, y)) {
                        continue;
                    }
                    perimeter += 1
                }
            }
            info!("[{c}{i}] area:{area} permiter:{perimeter}");
            region_dimensions.push((area, perimeter));
        }
    }
    info!("dimensions: {region_dimensions:?}");

    let price: usize = region_dimensions.iter().map(|(a, p)| a * p).sum();
    println!("{price}");
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    info!("grid: {grid:?}");

    let width = grid[0].len();
    let height = grid.len();

    let mut visited: Vec<Vec<bool>> = grid.iter().map(|r| r.iter().map(|_| false).collect()).collect();
    let mut region_mapping: HashMap<char, Vec<HashSet<(isize, isize)>>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            let c = grid[y][x];

            let mut region = HashSet::new();

            let mut queue = VecDeque::new();
            queue.push_back((x as isize, y as isize));

            while let Some((x, y)) = queue.pop_front() {
                region.insert((x, y));

                let directions = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                for (x, y) in directions {
                    if x < 0 || x >= width as isize {
                        continue;
                    }
                    if y < 0 || y >= height as isize {
                        continue;
                    }
                    if visited[y as usize][x as usize] {
                        continue;
                    }
                    if grid[y as usize][x as usize] != c {
                        continue;
                    }
                    visited[y as usize][x as usize] = true;
                    queue.push_front((x, y));
                }
            }
            info!("new region for '{c}': {region:?}");
            region_mapping
                .entry(c)
                .and_modify(|r| r.push(region.clone()))
                .or_insert(vec![region]);
        }
    }
    info!("visited: {visited:?}");
    info!("regions: {region_mapping:?}");

    let mut region_dimensions = Vec::new();
    for (c, regions) in region_mapping {
        for (i, region) in regions.iter().enumerate() {
            let area = region.len();
            let mut h_edges = BTreeSet::new();
            let mut v_edges = BTreeSet::new();
            for (x, y) in region.clone() {
                let h_directions = [(x, y - 1), (x, y + 1)];
                let v_directions = [(x - 1, y), (x + 1, y)];
                for (side, dir) in h_directions.iter().enumerate() {
                    if region.contains(dir) {
                        continue;
                    }
                    h_edges.insert((dir.1, side, dir.0));
                }
                for (side, dir) in v_directions.iter().enumerate() {
                    if region.contains(dir) {
                        continue;
                    }
                    v_edges.insert((dir.0, side, dir.1));
                }
            }
            info!("[{c}{i}] h_edges:{h_edges:?} (len:{})", h_edges.len());
            info!("[{c}{i}] v_edges:{v_edges:?} (len:{})", v_edges.len());
            info!("[{c}{i}] perimeter:{}", h_edges.len() + v_edges.len());

            let mut current = None;
            let mut h_sides = 0;
            for (y, side, x) in h_edges.iter() {
                let Some(cur) = current else {
                    info!("[{c}{i}] h_edge cur:(None) new:({y}, {x}) +1");
                    h_sides += 1;
                    current = Some((y, side, x));
                    continue;
                };
                current = Some((y, side, x));

                if cur.0 == y && cur.1 == side && cur.2 + 1 == *x {
                    info!(
                        "[{c}{i}] h_edge cur:({}, {}, {}) new:({y}, {x}) skip",
                        cur.0, cur.1, cur.2
                    );
                    continue;
                }
                info!(
                    "[{c}{i}] h_edge cur:({}, {}, {}) new:({y}, {x}) +1",
                    cur.0, cur.1, cur.2
                );
                h_sides += 1;
            }

            let mut current = None;
            let mut v_sides = 0;
            for (x, side, y) in v_edges.iter() {
                let Some(cur) = current else {
                    info!("[{c}{i}] v_edge cur:(None) new:({x}, {side}, {y}) +1");
                    v_sides += 1;
                    current = Some((x, side, y));
                    continue;
                };
                current = Some((x, side, y));

                if cur.0 == x && cur.1 == side && cur.2 + 1 == *y {
                    info!(
                        "[{c}{i}] v_edge cur:({}, {}, {}) new:({x}, {side}, {y}) skip",
                        cur.0, cur.1, cur.2
                    );
                    continue;
                }
                info!(
                    "[{c}{i}] v_edge cur:({}, {}, {}) new:({x}, {side}, {y}) +1",
                    cur.0, cur.1, cur.2
                );
                v_sides += 1;
            }

            info!("[{c}{i}] area:{area} h_sides:{h_sides}, v_sides:{v_sides}");
            region_dimensions.push((area, h_sides + v_sides));
        }
    }
    info!("dimensions: {region_dimensions:?}");

    let price: usize = region_dimensions.iter().map(|(a, p)| a * p).sum();
    println!("{price}");
    Ok(())
}
