use crate::utils::pos;
use crate::{solver::Solver, utils::generate_benchmark};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Position = crate::utils::Position<i16>;

#[derive(Debug)]
struct Antenna {
    position: Position,
}

impl Antenna {
    fn is_valid_frequency(frequency: char) -> bool {
        frequency.is_alphanumeric()
    }
}

#[derive(Debug)]
struct Map {
    antennas_by_frequency: HashMap<char, Vec<Antenna>>,
    map_size: i16,
}

impl Map {
    fn new(file: &str) -> Self {
        let mut antennas_by_frequency: HashMap<_, Vec<_>> = HashMap::default();
        let map_size = file.lines().next().unwrap().len() as i16;

        for (row, line) in file.lines().enumerate() {
            for (col, c) in line.char_indices() {
                if Antenna::is_valid_frequency(c) {
                    antennas_by_frequency.entry(c).or_default().push(Antenna {
                        position: pos!(row as i16, col as i16),
                    });
                }
            }
        }

        Self {
            antennas_by_frequency,
            map_size,
        }
    }

    fn compute_all_antinode_positions(
        &self,
        include_reasonant_harmonics: bool,
    ) -> HashSet<Position> {
        let mut antinode_positions = HashSet::default();

        for antennas in self.antennas_by_frequency.values() {
            for antennas_pair in antennas.iter().combinations(2) {
                if include_reasonant_harmonics {
                    antinode_positions.extend(antennas_pair.iter().map(|antenna| antenna.position));
                }

                antinode_positions.extend(self.compute_pair_of_antinode_positions(
                    antennas_pair[0],
                    antennas_pair[1],
                    include_reasonant_harmonics,
                ));
            }
        }

        antinode_positions
    }

    fn compute_pair_of_antinode_positions(
        &self,
        first: &Antenna,
        second: &Antenna,
        include_reasonant_harmonics: bool,
    ) -> Vec<Position> {
        let mut antinode_positions = Vec::new();

        let delta_row = second.position.row - first.position.row;
        let delta_col = second.position.col - first.position.col;

        for ((delta_row, delta_col), mut antinode_position) in [
            ((delta_row, delta_col), second.position),
            ((-delta_row, -delta_col), first.position),
        ] {
            loop {
                antinode_position.row += delta_row;
                antinode_position.col += delta_col;

                let in_bound = self.is_position_inbound(antinode_position);
                if in_bound {
                    antinode_positions.push(antinode_position);
                }

                if !in_bound || !include_reasonant_harmonics {
                    break;
                }
            }
        }

        antinode_positions
    }

    fn is_position_inbound(&self, position: Position) -> bool {
        (0..self.map_size).contains(&position.row) && (0..self.map_size).contains(&position.col)
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let map = Map::new(file);
        let antinode_positions = map.compute_all_antinode_positions(false);
        println!("We found {} antinode positions.", antinode_positions.len());
    }

    fn solve_part2(file: &str) {
        let map = Map::new(file);
        let antinode_positions = map.compute_all_antinode_positions(true);
        println!("We found {} antinode positions.", antinode_positions.len());
    }
}

generate_benchmark!(day8);
