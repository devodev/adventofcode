mod day01;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Advent of Code 2024 - Day 01.
    #[clap(visible_aliases = &["01", "1"])]
    Day01(day01::Args),
}

impl Args {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Day01(args) => args.run(),
        }
    }
}
