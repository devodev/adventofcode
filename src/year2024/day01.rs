use tracing::info;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Advent of Code 2024 - Day 01 - Historian Hysteria.
    HistorianHysteria,
}

impl Args {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::HistorianHysteria => hystorian_hysteria(),
        }
    }
}

fn hystorian_hysteria() -> anyhow::Result<()> {
    info!("Solving: Advent of Code 2024 - Day 01 - Historian Hysteria");
    Ok(())
}
