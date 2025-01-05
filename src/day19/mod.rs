use strum::EnumCount;
use strum_macros::EnumCount;

use crate::trie::{Trie, TrieElement};
use crate::{solver::Solver, utils::generate_benchmark};

#[derive(Clone, Copy, Debug, PartialEq, EnumCount)]
enum Stripe {
    White = 0,
    Blue = 1,
    Black = 2,
    Red = 3,
    Green = 4,
}

impl From<u8> for Stripe {
    fn from(value: u8) -> Self {
        match value {
            b'w' => Self::White,
            b'u' => Self::Blue,
            b'b' => Self::Black,
            b'r' => Self::Red,
            b'g' => Self::Green,
            _ => unreachable!(),
        }
    }
}

impl TrieElement for Stripe {
    fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug)]
struct TowelManager {
    patterns: Trie<Stripe, { Stripe::COUNT }>,
    desired_designs: Vec<Vec<Stripe>>,
}

impl TowelManager {
    fn new(file: &str) -> Self {
        let mut lines = file.lines();
        let patterns = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|pattern| pattern.bytes().map(Stripe::from))
            .collect();

        // Ignore empty line.
        lines.next();

        let desired_designs = lines
            .map(|line| line.bytes().map(Stripe::from).collect())
            .collect();

        Self {
            patterns,
            desired_designs,
        }
    }

    fn count_all_possible_designs(&mut self, count_unique_designs: bool) -> u64 {
        let mut count_possible_designs = 0;

        for design in &self.desired_designs {
            match self.patterns.count_all_word_arrangements(&design) {
                1.. if count_unique_designs => count_possible_designs += 1,
                count => count_possible_designs += count,
            }
        }

        count_possible_designs
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut towel_manager = TowelManager::new(file);
        println!(
            "The number of possible designs is {}",
            towel_manager.count_all_possible_designs(true)
        );
    }

    fn solve_part2(file: &str) {
        let mut towel_manager = TowelManager::new(file);
        println!(
            "The number of all possible design arrangements is {}",
            towel_manager.count_all_possible_designs(false)
        );
    }
}

generate_benchmark!(day19);
