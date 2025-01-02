use std::str::FromStr;

use crate::{
    solver::Solver,
    utils::{Position, generate_benchmark, pos, shard_and_solve_concurrently},
};
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROBOT: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

#[derive(Clone, Copy, Debug)]
struct Velocity {
    horizontal: i32,
    vertical: i32,
}

#[derive(Clone, Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (
            _,
            [
                horizontal_position,
                vertical_position,
                horizontal_velocity,
                vertical_velocity,
            ],
        ) = ROBOT
            .captures(s)
            .ok_or(anyhow!("capture failed"))?
            .extract();

        let horizontal_position = horizontal_position.parse()?;
        let vertical_position = vertical_position.parse()?;
        let position = pos!(vertical_position, horizontal_position);

        let horizontal_velocity = horizontal_velocity.parse()?;
        let vertical_velocity = vertical_velocity.parse()?;
        let velocity = Velocity {
            horizontal: horizontal_velocity,
            vertical: vertical_velocity,
        };

        Ok(Self { position, velocity })
    }
}

impl Robot {
    // Note: this is safe to call as long as `num_generations`, `num_horizontal_tiles`, and
    // `num_vertical_tiles` can all safely be represented as an i32.
    fn run(
        &self,
        num_generations: usize,
        num_horizontal_tiles: usize,
        num_vertical_tiles: usize,
    ) -> Self {
        let row_translation =
            (self.velocity.vertical * num_generations as i32).rem_euclid(num_vertical_tiles as i32);
        // Note: it is safe to cast `row_translation` from an i32 to a usize, because the value,
        // the result of rem_euclid, is always nonnegative.
        let final_row = (self.position.row + row_translation as usize) % num_vertical_tiles;

        let col_translation = (self.velocity.horizontal * num_generations as i32)
            .rem_euclid(num_horizontal_tiles as i32);
        // Note: it is safe to cast `col_translation` from an i32 to a usize, because the value,
        // the result of rem_euclid, is always nonnegative.
        let final_col = (self.position.col + col_translation as usize) % num_horizontal_tiles;

        Self {
            position: pos!(final_row, final_col),
            velocity: self.velocity,
        }
    }
}

#[derive(Clone, Debug)]
struct Simulation {
    robots: Vec<Robot>,
    num_horizontal_tiles: usize,
    num_vertical_tiles: usize,
    generation: usize,
}

impl Simulation {
    fn new(file: &str) -> Result<Self, anyhow::Error> {
        let mut lines = file.lines();
        let dimensions = lines.next().unwrap().split_once(',').unwrap();
        let (num_horizontal_tiles, num_vertical_tiles) =
            (dimensions.0.parse().unwrap(), dimensions.1.parse().unwrap());

        let robots = lines.map(Robot::from_str).collect::<Result<_, _>>()?;
        Ok(Self {
            robots,
            num_horizontal_tiles,
            num_vertical_tiles,
            generation: 0,
        })
    }

    fn run(&self, num_generations: usize) -> Self {
        let num_horizontal_tiles = self.num_horizontal_tiles;
        let num_vertical_tiles = self.num_vertical_tiles;
        let robots = self
            .robots
            .iter()
            .map(|robot| robot.run(num_generations, num_horizontal_tiles, num_vertical_tiles))
            .collect();

        Self {
            robots,
            num_horizontal_tiles,
            num_vertical_tiles,
            generation: self.generation + num_generations,
        }
    }

    fn calculate_safety_factor(&self) -> usize {
        let median_row = self.num_vertical_tiles / 2;
        let median_col = self.num_horizontal_tiles / 2;

        let mut num_robots_top_left_quadrant = 0;
        let mut num_robots_top_right_quadrant = 0;
        let mut num_robots_bottom_left_quadrant = 0;
        let mut num_robots_bottom_right_quadrant = 0;
        for robot in &self.robots {
            match robot.position {
                Position { row, col } if row < median_row && col < median_col => {
                    num_robots_top_left_quadrant += 1
                }
                Position { row, col } if row < median_row && col > median_col => {
                    num_robots_top_right_quadrant += 1
                }
                Position { row, col } if row > median_row && col < median_col => {
                    num_robots_bottom_left_quadrant += 1
                }
                Position { row, col } if row > median_row && col > median_col => {
                    num_robots_bottom_right_quadrant += 1
                }
                // A robot on the median row or median column is not part of any quadrants.
                _ => {}
            }
        }

        num_robots_top_left_quadrant
            * num_robots_top_right_quadrant
            * num_robots_bottom_left_quadrant
            * num_robots_bottom_right_quadrant
    }

    fn display_grid(&self) -> String {
        let mut grid = vec![vec![' '; self.num_horizontal_tiles]; self.num_vertical_tiles];

        for robot in &self.robots {
            grid[robot.position.row][robot.position.col] = 'X';
        }

        grid.iter()
            .map(String::from_iter)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let simulation = Simulation::new(file).unwrap().run(100);
        let safety_factor = simulation.calculate_safety_factor();
        println!("The safety factor is {safety_factor}.");
    }

    fn solve_part2(file: &str) {
        let simulation = Simulation::new(file).unwrap();

        // Find the generation with the minimum safety score and secondly minimum generation.
        // This is a clue that this image has less entropy, meaning a lot of robots are
        // concentrated in one quadrant. The grid with the minimum entropy indeed happens to
        // be the the one displaying a Christmas tree.
        let (min_safety_factor, min_generation) = shard_and_solve_concurrently(
            1..10000, // Ten thousand generations seems to be enough.
            simulation.clone(),
            |generations, simulation| {
                let mut min_safety_factor = usize::MAX;
                let mut min_simulation = None;

                for generation in generations {
                    let next_simulation = simulation.run(generation);
                    let next_safety_factor = next_simulation.calculate_safety_factor();
                    if next_safety_factor < min_safety_factor {
                        min_safety_factor = next_safety_factor;
                        min_simulation = Some(next_simulation);
                    }
                }

                return (min_safety_factor, min_simulation.unwrap().generation);
            },
        )
        .min()
        .unwrap();

        println!(
            "Safety factor: {min_safety_factor}\tGeneration: {}\n{}\n",
            min_generation,
            simulation.run(min_generation).display_grid()
        );
    }
}

generate_benchmark!(day14);
