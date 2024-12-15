use hashbrown::HashSet;

use crate::solver::Solver;
use crate::utils::{Position, generate_benchmark, pos};

const TRAIL_START: i8 = 0;
const TRAIL_END: i8 = 9;
const TRAIL_OUT_OF_BOUNDS: i8 = -1;

struct TopographicMap {
    topographic_map: Vec<Vec<i8>>,
    trailheads: Vec<Position>,
}

impl TopographicMap {
    fn new(file: &str) -> Self {
        let mut topographic_map = Vec::new();
        let mut trailheads = Vec::new();
        // Note that the map is padded with a layer of out_of_bounds locations.
        let topographic_map_size = file.lines().next().unwrap().len() + 2;

        topographic_map.push(vec![TRAIL_OUT_OF_BOUNDS; topographic_map_size]);
        for (row, line) in file.lines().enumerate() {
            let mut topographic_row = vec![TRAIL_OUT_OF_BOUNDS; 1];
            for (col, height) in line.char_indices() {
                let height = height.to_digit(10).unwrap() as i8;
                topographic_row.push(height);
                if height == TRAIL_START {
                    trailheads.push(pos!(row + 1, col + 1));
                }
            }
            topographic_row.push(TRAIL_OUT_OF_BOUNDS);
            topographic_map.push(topographic_row);
        }
        topographic_map.push(vec![TRAIL_OUT_OF_BOUNDS; topographic_map_size]);

        Self {
            topographic_map,
            trailheads,
        }
    }

    fn at(&self, position: Position) -> i8 {
        self.topographic_map[position.row][position.col]
    }

    fn compute_trailheads_score(&self, skip_duplicate_trailheads: bool) -> usize {
        self.trailheads
            .iter()
            .map(|trailhead| {
                Self::compute_trailhead_score(*trailhead, self, skip_duplicate_trailheads)
            })
            .sum()
    }

    fn compute_trailhead_score(
        trailhead: Position,
        topographic_map: &Self,
        skip_duplicate_trailheads: bool,
    ) -> usize {
        let mut visited_positions = HashSet::new();
        let mut positions_to_visit = vec![trailhead];

        let mut trailheads_count = 0;
        while let Some(current_position) = positions_to_visit.pop() {
            if skip_duplicate_trailheads && !visited_positions.insert(current_position) {
                continue;
            }

            let current_height = topographic_map.at(current_position);
            if current_height == TRAIL_END {
                trailheads_count += 1;
                continue;
            }

            for next_position in current_position.surroundings() {
                let next_height = topographic_map.at(next_position);
                if next_height == current_height + 1 {
                    positions_to_visit.push(next_position);
                }
            }
        }

        trailheads_count
    }
}
pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let topographic_map = TopographicMap::new(file);
        let trailheads_scores = topographic_map.compute_trailheads_score(true);
        println!("The trailheads score is {}", trailheads_scores);
    }

    fn solve_part2(file: &str) {
        let topographic_map = TopographicMap::new(file);
        let trailheads_rating = topographic_map.compute_trailheads_score(false);
        println!("The trailheads rating is {}", trailheads_rating);
    }
}

generate_benchmark!(day10);
