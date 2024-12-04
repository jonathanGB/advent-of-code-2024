use crate::solver::Solver;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL: Regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    static ref MUL_WITH_DO_DONT: Regex =
        Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: String) {
        let sum_of_muls: i32 = MUL
            .captures_iter(&file)
            .map(|capture| {
                capture.name("a").unwrap().as_str().parse::<i32>().unwrap()
                    * capture.name("b").unwrap().as_str().parse::<i32>().unwrap()
            })
            .sum();

        println!("Sum of muls: {sum_of_muls}");
    }

    fn solve_part2(file: String) {
        let mut enabled = true;
        let sum_of_muls: i32 = MUL_WITH_DO_DONT
            .captures_iter(&file)
            .map(|capture| {
                if capture.name("do").is_some() {
                    enabled = true;
                    0
                } else if capture.name("dont").is_some() {
                    enabled = false;
                    0
                } else if !enabled {
                    0
                } else {
                    capture.name("a").unwrap().as_str().parse::<i32>().unwrap()
                        * capture.name("b").unwrap().as_str().parse::<i32>().unwrap()
                }
            })
            .sum();

        println!("Sum of muls: {sum_of_muls}");
    }
}
