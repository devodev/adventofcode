mod year2024;

use std::io::Read;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

/// devodev's Advent of Code solver CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// The filepath of the input file, or `-` to read stdin.
    #[arg(short, long, default_value = "-")]
    input: String,

    /// Enable info(-v), debug(-vv) or trace(-vvv) logging
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Advent of Code 2024.
    #[command(visible_alias = "2024")]
    Year2024(year2024::Args),
}

impl Cli {
    #[tracing::instrument(skip_all)]
    fn read_input(&self) -> Result<String> {
        let input = match self.input.as_str() {
            "" => return Err(anyhow!("input flag cannot be empty")),
            "-" => {
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf).context("reading stdin")?;
                buf
            }
            path => std::fs::read_to_string(&self.input).with_context(|| format!("reading input file '{path}'"))?,
        };
        Ok(input)
    }

    #[tracing::instrument(skip_all)]
    fn run(self, input: String) -> Result<()> {
        match self.command {
            Commands::Year2024(args) => args.run(input),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_tracing(cli.verbose);

    let input = cli.read_input()?;
    match cli.run(input) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("{err:#}")),
    }
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
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(stderr_subscriber).expect("setting tracing global default failed");
}
