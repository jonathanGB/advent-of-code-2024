use clap::Parser;

mod args;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solver;

use args::{Args, Day};
use day1::Day1Solver;
use day2::Day2Solver;
use day3::Day3Solver;
use day4::Day4Solver;
use day5::Day5Solver;
use day6::Day6Solver;
use day7::Day7Solver;
use day8::Day8Solver;
use day9::Day9Solver;
use day10::Day10Solver;
use day11::Day11Solver;
use day12::Day12Solver;
use day13::Day13Solver;
use day14::Day14Solver;
use day15::Day15Solver;
use day16::Day16Solver;
use day17::Day17Solver;
use day18::Day18Solver;
use day19::Day19Solver;
use day20::Day20Solver;
use day21::Day21Solver;
use day22::Day22Solver;
use day23::Day23Solver;
use day24::Day24Solver;
use day25::Day25Solver;
use solver::Solver;

fn main() {
    let cli = Args::parse();

    match cli.day {
        Day::Day1 { part } => Day1Solver::solve(part),
        Day::Day2 { part } => Day2Solver::solve(part),
        Day::Day3 { part } => Day3Solver::solve(part),
        Day::Day4 { part } => Day4Solver::solve(part),
        Day::Day5 { part } => Day5Solver::solve(part),
        Day::Day6 { part } => Day6Solver::solve(part),
        Day::Day7 { part } => Day7Solver::solve(part),
        Day::Day8 { part } => Day8Solver::solve(part),
        Day::Day9 { part } => Day9Solver::solve(part),
        Day::Day10 { part } => Day10Solver::solve(part),
        Day::Day11 { part } => Day11Solver::solve(part),
        Day::Day12 { part } => Day12Solver::solve(part),
        Day::Day13 { part } => Day13Solver::solve(part),
        Day::Day14 { part } => Day14Solver::solve(part),
        Day::Day15 { part } => Day15Solver::solve(part),
        Day::Day16 { part } => Day16Solver::solve(part),
        Day::Day17 { part } => Day17Solver::solve(part),
        Day::Day18 { part } => Day18Solver::solve(part),
        Day::Day19 { part } => Day19Solver::solve(part),
        Day::Day20 { part } => Day20Solver::solve(part),
        Day::Day21 { part } => Day21Solver::solve(part),
        Day::Day22 { part } => Day22Solver::solve(part),
        Day::Day23 { part } => Day23Solver::solve(part),
        Day::Day24 { part } => Day24Solver::solve(part),
        Day::Day25 { part } => Day25Solver::solve(part),
    }
}
