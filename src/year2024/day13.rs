use std::ops::Rem;

use anyhow::Result;
use regex::Regex;
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
    let button_re = Regex::new(r"Button ([AB]): X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut problems = Vec::new();
    let mut current_problem = Problem::default();
    for line in input.lines() {
        if let Some(cap) = button_re.captures(line) {
            let (_, [ab, x, y]) = cap.extract();
            let button = (x.parse().unwrap(), y.parse().unwrap());
            match ab {
                "A" => current_problem.button_a = button,
                "B" => current_problem.button_b = button,
                v => panic!("invalid Button: {v}"),
            }
            continue;
        };
        if let Some(cap) = prize_re.captures(line) {
            let (_, [x, y]) = cap.extract();
            current_problem.prize = (x.parse().unwrap(), y.parse().unwrap());
            problems.push(current_problem);
            current_problem = Problem::default();
        };
    }
    info!("problems: {problems:?}");

    let solved: Vec<_> = problems.iter().map(|p| p.solve()).collect();
    info!("solved: {solved:?}");

    let costs = (3, 1);
    let tokens: i32 = solved
        .into_iter()
        .flatten()
        .map(|(a, b)| (a * costs.0) + (b * costs.1))
        .sum();

    println!("{tokens}");
    Ok(())
}

fn part2(_input: String) -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, Default)]
struct Problem {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

impl Problem {
    // Solve two linear equations:
    //   X1a + X2b = pX
    //   Y1a + Y2b = pY
    //
    // Example:
    //   Button A: X+17, Y+86
    //   Button B: X+84, Y+37
    //   Prize: X=7870, Y=6450
    //
    //   17a (2,397) + 84b (11340) = 7870 for x
    //   86a + 37b = 6450 for y
    //
    // Cramer's Rule:
    // https://en.wikipedia.org/wiki/Cramer%27s_rule
    //
    // NOTE: a1b2 − b1a2 must be nonzero.
    //
    // |a1 b1| |x| = |c1|
    // |a2 b2| |y| = |c2|
    //
    //     |c1 b1|
    //     |c2 b2|   c1b2 - b1c2
    // a = ------- = -----------
    //     |a1 b1|   a1b2 - b1a2
    //     |a2 b2|
    //
    // c1b2(291190) - b1c2(541800) = -250610
    // ----------------------------   ------- = 38 = a
    // a1b2(629)    - b1a2(7224)   = -6,595
    //
    //     |a1 c1|
    //     |a2 c2|   a1c2 - c1a2
    // b = ------- = -----------
    //     |a1 b1|   a1b2 - b1a2
    //     |a2 b2|
    //
    // a1c2(109650) - c1a2(676820)  = -567170
    // ---------------------------- = ------- = 86 = b
    // a1b2(629)    - b1a2(7224)    = -6,595
    fn solve(&self) -> Option<(i32, i32)> {
        cramers_rule(
            (self.button_a.0, self.button_b.0, self.prize.0),
            (self.button_a.1, self.button_b.1, self.prize.1),
        )
    }
}

// In linear algebra, Cramer's rule is an explicit formula for the solution of a system of linear
// equations with as many equations as unknowns, valid whenever the system has a unique solution.
// https://en.wikipedia.org/wiki/Cramer%27s_rule
fn cramers_rule(eq1: (i32, i32, i32), eq2: (i32, i32, i32)) -> Option<(i32, i32)> {
    let (a1, b1, c1) = eq1;
    let (a2, b2, c2) = eq2;

    let denom = (a1 * b2) - (b1 * a2);
    info!("[{eq1:?} {eq2:?}] denom: {denom}");
    if denom == 0 {
        return None;
    }

    let x_nom = (c1 * b2) - (b1 * c2);
    let y_nom = (a1 * c2) - (c1 * a2);
    info!("[{eq1:?} {eq2:?}] x_nom: {x_nom}");
    info!("[{eq1:?} {eq2:?}] y_nom: {y_nom}");
    if x_nom == 0 || y_nom == 0 {
        return None;
    }

    let x_nom_rem = x_nom.rem(denom);
    info!("[{eq1:?} {eq2:?}] x_nom_rem: {x_nom_rem}");
    if x_nom_rem != 0 {
        return None;
    }
    let y_nom_rem = y_nom.rem(denom);
    info!("[{eq1:?} {eq2:?}] y_nom_rem: {y_nom_rem}");
    if y_nom_rem != 0 {
        return None;
    }

    let x = x_nom / denom;
    let y = y_nom / denom;

    Some((x, y))
}
