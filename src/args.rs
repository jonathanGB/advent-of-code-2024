use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[command(subcommand)]
    pub day: Day,
}

#[derive(Subcommand, Debug)]
pub enum Day {
    /// Solve day 1's problem.
    Day1 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 2's problem.
    Day2 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 3's problem.
    Day3 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 4's problem.
    Day4 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 5's problem.
    Day5 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 6's problem.
    Day6 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 7's problem.
    Day7 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 8's problem.
    Day8 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 9's problem.
    Day9 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 10's problem.
    Day10 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 11's problem.
    Day11 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 12's problem.
    Day12 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 13's problem.
    Day13 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 14's problem.
    Day14 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 15's problem.
    Day15 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 16's problem.
    Day16 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 17's problem.
    Day17 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 18's problem.
    Day18 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 19's problem.
    Day19 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 20's problem.
    Day20 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 21's problem.
    Day21 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 22's problem.
    Day22 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 23's problem.
    Day23 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 24's problem.
    Day24 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 25's problem.
    Day25 {
        #[command(subcommand)]
        part: Part,
    },
}

#[derive(Subcommand, Debug)]
pub enum Part {
    /// Solve the 1st part of the problem.
    Part1,
    /// Solve the 2nd part of the problem.
    Part2,
}
