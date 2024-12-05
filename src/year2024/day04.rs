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

// We know the input is ASCII, therefore we can index the string using its bytes representation.
fn part1(input: String) -> Result<()> {
    let mut x_hor = 0;
    let mut x_ver = 0;
    let mut x_dial = 0;
    let mut x_diar = 0;
    let mut s_hor = 0;
    let mut s_ver = 0;
    let mut s_dial = 0;
    let mut s_diar = 0;

    let mut count = 0;
    let lines: Vec<_> = input.lines().collect();
    for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len() {
            match &line[x..x + 1] {
                // check for XMAS
                "X" => {
                    // horizontal
                    if x < line.len() - 3 && &line[x + 1..x + 4] == "MAS" {
                        count += 1;
                        x_hor += 1;
                    }
                    // vertical
                    if y < lines.len() - 3
                        && (
                            &lines[y + 1][x..x + 1],
                            &lines[y + 2][x..x + 1],
                            &lines[y + 3][x..x + 1],
                        ) == ("M", "A", "S")
                    {
                        count += 1;
                        x_ver += 1;
                    }
                    // diagonal left \
                    if x < line.len() - 3
                        && y < lines.len() - 3
                        && (
                            &lines[y + 1][x + 1..x + 2],
                            &lines[y + 2][x + 2..x + 3],
                            &lines[y + 3][x + 3..x + 4],
                        ) == ("M", "A", "S")
                    {
                        count += 1;
                        x_dial += 1;
                    }
                    // diagonal right /
                    if x > 2
                        && y < lines.len() - 3
                        && (
                            &lines[y + 1][x - 1..x],
                            &lines[y + 2][x - 2..x - 1],
                            &lines[y + 3][x - 3..x - 2],
                        ) == ("M", "A", "S")
                    {
                        count += 1;
                        x_diar += 1;
                    }
                }
                // check for SAMX
                "S" => {
                    // horizontal
                    if x < line.len() - 3 && &line[x + 1..x + 4] == "AMX" {
                        count += 1;
                        s_hor += 1;
                    }
                    // vertical
                    if y < lines.len() - 3
                        && (
                            &lines[y + 1][x..x + 1],
                            &lines[y + 2][x..x + 1],
                            &lines[y + 3][x..x + 1],
                        ) == ("A", "M", "X")
                    {
                        count += 1;
                        s_ver += 1;
                    }
                    // diagonal left \
                    if x < line.len() - 3
                        && y < lines.len() - 3
                        && (
                            &lines[y + 1][x + 1..x + 2],
                            &lines[y + 2][x + 2..x + 3],
                            &lines[y + 3][x + 3..x + 4],
                        ) == ("A", "M", "X")
                    {
                        count += 1;
                        s_dial += 1;
                    }
                    // diagonal right /
                    if x > 2
                        && y < lines.len() - 3
                        && (
                            &lines[y + 1][x - 1..x],
                            &lines[y + 2][x - 2..x - 1],
                            &lines[y + 3][x - 3..x - 2],
                        ) == ("A", "M", "X")
                    {
                        count += 1;
                        s_diar += 1;
                    }
                }
                _ => {}
            }
        }
    }
    info!(
        "X(hor:{x_hor} ver:{x_ver} dial:{x_dial} diar:{x_diar}) S(hor:{s_hor} ver:{s_ver} dial:{s_dial} diar:{s_diar})"
    );
    println!("{count}");

    Ok(())
}

fn part2(input: String) -> Result<()> {
    let mut count = 0;
    let lines: Vec<_> = input.lines().collect();
    for y in 0..lines.len() - 2 {
        for x in 0..lines[y].len() - 2 {
            match (
                &lines[y][x..x + 1],
                &lines[y][x + 2..x + 3],
                &lines[y + 1][x + 1..x + 2],
                &lines[y + 2][x..x + 1],
                &lines[y + 2][x + 2..x + 3],
            ) {
                ("M", "M", "A", "S", "S")
                | ("S", "S", "A", "M", "M")
                | ("S", "M", "A", "S", "M")
                | ("M", "S", "A", "M", "S") => count += 1,
                _ => {}
            }
        }
    }
    println!("{count}");

    Ok(())
}
