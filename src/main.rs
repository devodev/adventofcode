mod year2024;

use clap::{Parser, Subcommand};
use tracing::level_filters::LevelFilter;

/// devodev's Advent of Code solver CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable info(-v), debug(-vv) or trace(-vvv) logging
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Advent of Code 2024.
    #[clap(visible_alias = "2024")]
    Year2024(year2024::Args),
}

impl Cli {
    fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Year2024(args) => args.run(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    setup_tracing(cli.verbose);
    cli.run()
}

fn setup_tracing(verbose: u8) {
    let level = match verbose {
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        3 => LevelFilter::TRACE,
        _ => LevelFilter::WARN,
    };
    let stderr_subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(stderr_subscriber).expect("setting tracing global default failed");
}
