mod day01;
mod day02;
mod day03;
mod day04;

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
}

impl Args {
    pub fn run(self, input: String) -> Result<()> {
        match self.command {
            Commands::Day01(args) => args.run(input),
            Commands::Day02(args) => args.run(input),
            Commands::Day03(args) => args.run(input),
            Commands::Day04(args) => args.run(input),
        }
    }
}
