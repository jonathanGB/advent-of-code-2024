use std::num::ParseIntError;
use std::str::FromStr;

use hashbrown::HashMap;

use crate::solver::Solver;
use crate::utils::generate_benchmark;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Stone {
    value: u64,
    generation: u8,
}
impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse()?,
            generation: 0,
        })
    }
}
impl Stone {
    fn next(&self) -> Vec<Self> {
        let generation = self.generation + 1;
        let num_digits = if self.value == 0 {
            1
        } else {
            self.value.ilog10() + 1
        };

        if self.value == 0 {
            vec![Self {
                value: 1,
                generation,
            }]
        } else if num_digits % 2 == 0 {
            let exponent = 10_u64.pow(num_digits >> 1);
            let left_number = self.value / exponent;
            let right_number = self.value % exponent;
            vec![
                Self {
                    value: left_number,
                    generation,
                },
                Self {
                    value: right_number,
                    generation,
                },
            ]
        } else {
            vec![Self {
                value: self.value * 2024,
                generation,
            }]
        }
    }
}

struct Blinker {
    stones: Vec<Stone>,
}
impl Blinker {
    fn new(file: &str) -> Self {
        Self {
            stones: file
                .split_whitespace()
                .map(Stone::from_str)
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }

    fn blink(self, final_generation: u8) -> u64 {
        let mut stones_history = HashMap::default();

        self.stones
            .into_iter()
            .map(|stone| Self::blink_rec(stone, &mut stones_history, final_generation))
            .sum()
    }

    fn blink_rec(
        stone: Stone,
        stones_history: &mut HashMap<Stone, u64>,
        final_generation: u8,
    ) -> u64 {
        if stone.generation == final_generation {
            return 1;
        }

        if let Some(num_stones) = stones_history.get(&stone) {
            return *num_stones;
        }

        let num_stones = stone
            .next()
            .into_iter()
            .map(|next_stone| Self::blink_rec(next_stone, stones_history, final_generation))
            .sum();

        stones_history.insert(stone, num_stones);
        return num_stones;
    }
}
pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let blinker = Blinker::new(file);
        println!("We have {} stones", blinker.blink(25));
    }

    fn solve_part2(file: &str) {
        let blinker = Blinker::new(file);
        println!("We have {} stones", blinker.blink(75));
    }
}

generate_benchmark!(day11);
