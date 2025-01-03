mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use anyhow::Result;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Advent of Code 2024 - Day 01 - Historian Hysteria.
    #[command(visible_aliases = &["day1", "01", "1"])]
    Day01(day01::Args),
    /// Advent of Code 2024 - Day 02 - Red-Nosed Reports.
    #[command(visible_aliases = &["day2", "02", "2"])]
    Day02(day02::Args),
    /// Advent of Code 2024 - Day 03 - Mull It Over.
    #[command(visible_aliases = &["day3", "03", "3"])]
    Day03(day03::Args),
    /// Advent of Code 2024 - Day 04 - Ceres Search.
    #[command(visible_aliases = &["day4", "04", "4"])]
    Day04(day04::Args),
    /// Advent of Code 2024 - Day 05 - Print Queue.
    #[command(visible_aliases = &["day5", "05", "5"])]
    Day05(day05::Args),
    /// Advent of Code 2024 - Day 06 - Guard Gallivant.
    #[command(visible_aliases = &["day6", "06", "6"])]
    Day06(day06::Args),
    /// Advent of Code 2024 - Day 07 - Bridge Repair.
    #[command(visible_aliases = &["day7", "07", "7"])]
    Day07(day07::Args),
    /// Advent of Code 2024 - Day 08 - Resonant Collinearity.
    #[command(visible_aliases = &["day8", "08", "8"])]
    Day08(day08::Args),
    /// Advent of Code 2024 - Day 09 - Disk Fragmenter.
    #[command(visible_aliases = &["day9", "09", "9"])]
    Day09(day09::Args),
    /// Advent of Code 2024 - Day 10 - Hoof It.
    #[command(visible_aliases = &["day10", "10", "10"])]
    Day10(day10::Args),
    /// Advent of Code 2024 - Day 11 - Plutonian Pebbles.
    #[command(visible_aliases = &["day11", "11", "11"])]
    Day11(day11::Args),
}

impl Args {
    pub fn run(self, input: String) -> Result<()> {
        match self.command {
            Commands::Day01(args) => args.run(input),
            Commands::Day02(args) => args.run(input),
            Commands::Day03(args) => args.run(input),
            Commands::Day04(args) => args.run(input),
            Commands::Day05(args) => args.run(input),
            Commands::Day06(args) => args.run(input),
            Commands::Day07(args) => args.run(input),
            Commands::Day08(args) => args.run(input),
            Commands::Day09(args) => args.run(input),
            Commands::Day10(args) => args.run(input),
            Commands::Day11(args) => args.run(input),
        }
    }
}
