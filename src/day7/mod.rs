use std::{str::FromStr, sync::mpsc::channel};

use anyhow::anyhow;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::solver::Solver;

#[derive(Clone, Debug, EnumIter, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

impl Operator {
    fn is_concatenation(&self) -> bool {
        self == &Self::Concatenation
    }
}

#[derive(Debug)]
struct Equation {
    value: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn try_compute(&self, operators: Vec<Operator>) -> Option<()> {
        let mut result = self.operands[0];
        for (right_operand, operator) in self.operands.iter().skip(1).zip(operators.into_iter()) {
            result = match operator {
                Operator::Add => result.checked_add(*right_operand),
                Operator::Multiply => result.checked_mul(*right_operand),
                Operator::Concatenation => {
                    let right_operand_num_digits = 1 + right_operand.checked_ilog10()?;
                    result
                        .checked_mul(10_i64.checked_pow(right_operand_num_digits)?)?
                        .checked_add(*right_operand)
                }
            }?;
        }

        if result == self.value { Some(()) } else { None }
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, operands) = s
            .split_once(": ")
            .ok_or(anyhow!("Could not split the value from the operands"))?;
        let operands = operands
            .split_whitespace()
            .map(|operand| operand.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            value: value.parse()?,
            operands,
        })
    }
}

pub struct SolverImpl {}

impl SolverImpl {
    fn solve<I>(file: &str, operators: I)
    where
        I: Iterator<Item = Operator> + Clone + Send + 'static,
    {
        let (tx, rx) = channel();
        let available_parallelism = std::thread::available_parallelism().unwrap().get();
        let mut sharded_lines = vec![Vec::new(); available_parallelism];
        for (i, line) in file.lines().enumerate() {
            sharded_lines[i % available_parallelism].push(line.to_string());
        }

        for sharded_line in sharded_lines {
            let tx = tx.clone();
            let operators = operators.clone();

            std::thread::spawn(move || {
                let mut local_total_calibration_result = 0;

                'equations: for line in sharded_line {
                    let equation: Equation = line.parse().unwrap();
                    let num_operators = equation.operands.len() - 1;

                    for tentative_operators in (0..num_operators)
                        .map(|_| operators.clone())
                        .multi_cartesian_product()
                    {
                        if equation.try_compute(tentative_operators).is_some() {
                            local_total_calibration_result += equation.value;
                            continue 'equations;
                        }
                    }
                }

                tx.send(local_total_calibration_result).unwrap();
            });
        }

        let mut total_calibration_result = 0;

        for _ in 0..available_parallelism {
            total_calibration_result += rx.recv().unwrap();
        }

        println!("The total calibration result is {total_calibration_result}");
    }
}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        Self::solve(
            file,
            Operator::iter().filter(|operator| !operator.is_concatenation()),
        );
    }

    fn solve_part2(file: &str) {
        Self::solve(file, Operator::iter());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_day7_part1(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day7/input.txt").unwrap();

        b.iter(|| SolverImpl::solve_part1(&file));
    }

    #[bench]
    fn bench_day7_part2(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day7/input.txt").unwrap();

        b.iter(|| SolverImpl::solve_part2(&file));
    }
}
