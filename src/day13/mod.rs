use itertools::Itertools;

use crate::{solver::Solver, utils::generate_benchmark};

const EPSILON: f64 = 0.0001;
const NUM_TOKENS_PER_A_PRESS: u64 = 3;
const NUM_TOKENS_PER_B_PRESS: u64 = 1;

pub struct SolverImpl {}

#[derive(Debug)]
struct ClawMachine {
    xa: f64,
    xb: f64,
    xf: f64,
    ya: f64,
    yb: f64,
    yf: f64,
}

impl ClawMachine {
    fn find_num_tokens_spent(&self) -> u64 {
        let b_divisor = -self.xb * self.ya / self.xa + self.yb;
        if b_divisor == 0.0 {
            return 0;
        }

        let b_dividend = self.yf - (self.xf * self.ya / self.xa);
        let b_presses = b_dividend / b_divisor;
        let b_presses_approx = b_presses.round();

        if b_presses < b_presses_approx {
            if b_presses_approx - b_presses > EPSILON {
                return 0;
            }
        } else if b_presses > b_presses_approx {
            if b_presses - b_presses_approx > EPSILON {
                return 0;
            }
        }

        if b_presses_approx < 0.0 {
            return 0;
        }

        let a_presses = (self.xf - b_presses_approx * self.xb) / self.xa;
        let a_presses_approx = a_presses.round();

        if a_presses < a_presses_approx {
            if a_presses_approx - a_presses > EPSILON {
                return 0;
            }
        } else if a_presses > a_presses_approx {
            if a_presses - a_presses_approx > EPSILON {
                return 0;
            }
        }

        if a_presses_approx < 0.0 {
            return 0;
        }

        NUM_TOKENS_PER_B_PRESS * b_presses_approx as u64
            + NUM_TOKENS_PER_A_PRESS * a_presses_approx as u64
    }
}

#[derive(Debug)]
struct ClawMachineSimulation {
    claw_machines: Vec<ClawMachine>,
}

impl ClawMachineSimulation {
    fn new(file: &str, prize_position_offset: f64) -> Self {
        let mut claw_machines = Vec::new();

        for mut simulation in &file.lines().chunks(4) {
            let a_simulation = simulation.next().unwrap();
            let b_simulation = simulation.next().unwrap();
            let prize_simulation = simulation.next().unwrap();

            let (_, a_simulation) = a_simulation.split_once("X+").unwrap();
            let (xa, ya) = a_simulation.split_once(", Y+").unwrap();
            let (xa, ya) = (xa.parse().unwrap(), ya.parse().unwrap());

            let (_, b_simulation) = b_simulation.split_once("X+").unwrap();
            let (xb, yb) = b_simulation.split_once(", Y+").unwrap();
            let (xb, yb) = (xb.parse().unwrap(), yb.parse().unwrap());

            let (_, prize_simulation) = prize_simulation.split_once("X=").unwrap();
            let (xf, yf) = prize_simulation.split_once(", Y=").unwrap();
            let (xf, yf) = (
                xf.parse::<f64>().unwrap() + prize_position_offset,
                yf.parse::<f64>().unwrap() + prize_position_offset,
            );

            claw_machines.push(ClawMachine {
                xa,
                xb,
                xf,
                ya,
                yb,
                yf,
            });
        }

        Self { claw_machines }
    }

    fn find_num_tokens_spent(&self) -> u64 {
        self.claw_machines
            .iter()
            .map(|claw_machine| claw_machine.find_num_tokens_spent())
            .sum()
    }
}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let claw_machine_simulation = ClawMachineSimulation::new(file, 0.0);
        println!(
            "Number of tokens spent: {}",
            claw_machine_simulation.find_num_tokens_spent()
        );
    }

    fn solve_part2(file: &str) {
        let claw_machine_simulation = ClawMachineSimulation::new(file, 10000000000000.0);
        println!(
            "Number of tokens spent: {}",
            claw_machine_simulation.find_num_tokens_spent()
        );
    }
}

generate_benchmark!(day13);
