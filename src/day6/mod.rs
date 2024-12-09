use crate::solver::Solver;
use crate::utils::Position;
use hashbrown::HashSet;
use std::sync::mpsc::channel;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Visited,
    Unvisited,
    Obstructed,
    Outside,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Unvisited,
            '^' => Self::Visited,
            '#' => Self::Obstructed,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn is_visited(&self) -> bool {
        self == &Self::Visited
    }

    fn is_unvisited(&self) -> bool {
        self == &Self::Unvisited
    }

    fn is_obstructed(&self) -> bool {
        self == &Self::Obstructed
    }

    fn is_outside(&self) -> bool {
        self == &Self::Outside
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    // Moves the guard one tile following its patrol protocol.
    // That is, try to move one tile into the current direction. If the new tile is obstructed, rotate to the right,
    // and try in that new direction. Stops there if the tile to the right is also obstructed.
    // Returns true if the guard is still patrolling, aka it is not out of bounds. Otherwise, returns false.
    fn patrol(&mut self, lab: &Vec<Vec<Tile>>) -> bool {
        let Position { row, col } = self.position;
        if lab[row][col].is_outside() {
            return false;
        }

        let (new_position, (alternative_new_position, alternative_new_direction)) =
            match self.direction {
                Direction::Up => (
                    Position { row: row - 1, col },
                    (Position { row, col: col + 1 }, Direction::Right),
                ),
                Direction::Right => (
                    Position { row, col: col + 1 },
                    (Position { row: row + 1, col }, Direction::Down),
                ),
                Direction::Down => (
                    Position { row: row + 1, col },
                    (Position { row, col: col - 1 }, Direction::Left),
                ),
                Direction::Left => (
                    Position { row, col: col - 1 },
                    (Position { row: row - 1, col }, Direction::Up),
                ),
            };

        let new_tile = lab[new_position.row][new_position.col];
        let alternative_new_tile = lab[alternative_new_position.row][alternative_new_position.col];
        if new_tile.is_outside() {
            false
        } else if !new_tile.is_obstructed() {
            self.position = new_position;
            true
        } else if alternative_new_tile.is_outside() {
            false
        } else if alternative_new_tile.is_obstructed() {
            // Obstructed twice. Give up for now, maybe we'll get out of here on a subsequent patrol.
            self.direction = alternative_new_direction;
            true
        } else {
            self.position = alternative_new_position;
            self.direction = alternative_new_direction;
            true
        }
    }
}

#[derive(Clone, Debug)]
struct LabSimulation {
    // Note that the lab is padded all around with "outside" tiles.
    lab: Vec<Vec<Tile>>,
    guard: Guard,
    visited_tiles: HashSet<Position>,
    previous_guards: HashSet<Guard>,
}

impl LabSimulation {
    fn new(file: &str) -> Self {
        let mut lab = Vec::new();
        let mut guard_position = None;

        // Top empty row for the "outside" tiles.
        lab.push(Vec::new());

        for (mut row, line) in file.lines().enumerate() {
            // Plus one to include the top "outside" row.
            row += 1;

            // Add an "outside" tile to the left of the lab.
            lab.push(vec![Tile::Outside; 1]);

            for (mut col, tile) in line.chars().enumerate() {
                // Plus one to include the left "outside" column.
                col += 1;

                let tile: Tile = tile.into();
                if tile.is_visited() {
                    guard_position = Some(Position { row, col });
                }

                lab[row].push(tile);
            }

            // Add an "outside" tile to the right of the lab.
            lab[row].push(Tile::Outside);
        }

        // Minus one to discard the top "outside" row.
        let lab_size = lab.len() - 1;
        // Populate the top "outside" row now that we know its size.
        // Plus two to include the left "outside" column and the right "outside" column.
        lab[0].extend(std::iter::repeat_n(Tile::Outside, lab_size + 2));
        // Populate the bottom "outside" row now that we know its size.
        // Plus two to include the left "outside" column and the right "outside" column.
        lab.push(vec![Tile::Outside; lab_size + 2]);

        let position = guard_position.unwrap();
        let guard = Guard {
            position,
            direction: Direction::Up,
        };

        Self {
            lab,
            guard,
            visited_tiles: HashSet::from([position]),
            previous_guards: HashSet::from([guard]),
        }
    }

    fn at(&self, position: Position) -> &Tile {
        &self.lab[position.row][position.col]
    }

    fn at_mut(&mut self, position: Position) -> &mut Tile {
        &mut self.lab[position.row][position.col]
    }

    // Runs the guard patrol, and returns the set of tiles visited by the guard
    // until it exited the lab. Returns None if the guard got stuck in a loop.
    fn run_guard_patrol(mut self) -> Option<HashSet<Position>> {
        while self.guard.patrol(&self.lab) {
            let guard_position = self.guard.position;

            if self.at(guard_position).is_unvisited() {
                self.visited_tiles.insert(guard_position);
                *self.at_mut(guard_position) = Tile::Visited;
            } else if self.previous_guards.contains(&self.guard) {
                // The guard has previously been at this position looking in
                // the very same direction. This is a loop, exit!
                return None;
            }

            self.previous_guards.insert(self.guard);
        }

        Some(self.visited_tiles)
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let lab_simulation = LabSimulation::new(file);
        let unique_visited_tiles = lab_simulation.run_guard_patrol().unwrap();
        println!(
            "The guard visited {} unique tiles.",
            unique_visited_tiles.len()
        );
    }

    fn solve_part2(file: &str) {
        let lab_simulation = LabSimulation::new(file);
        let initial_guard_position = lab_simulation.guard.position;
        let mut potential_obstruction_sites = lab_simulation.clone().run_guard_patrol().unwrap();
        // Problem states that the initial guard position cannot be a potential obstruction site.
        potential_obstruction_sites.remove(&initial_guard_position);

        // The problem has over 5k potential sites, so we will shard them uniformly across buckets based on the machine's
        // available parallelism. Each shard will send their local count via the MPSC channel.
        let (tx, rx) = channel();
        let available_parallelism = std::thread::available_parallelism().unwrap().get();
        let mut sharded_potential_obstruction_sites = vec![Vec::new(); available_parallelism];
        for (i, potential_obstruction_site) in potential_obstruction_sites.into_iter().enumerate() {
            sharded_potential_obstruction_sites[i % available_parallelism]
                .push(potential_obstruction_site);
        }

        for sharded_potential_obstruction_site in sharded_potential_obstruction_sites {
            let lab_simulation = lab_simulation.clone();
            let tx = tx.clone();
            std::thread::spawn(move || {
                let mut sharded_count_loopable_configurations = 0;
                for potential_obstruction_site in sharded_potential_obstruction_site {
                    let mut tentative_lab_simulation = lab_simulation.clone();
                    *tentative_lab_simulation.at_mut(potential_obstruction_site) = Tile::Obstructed;

                    if tentative_lab_simulation.run_guard_patrol().is_none() {
                        sharded_count_loopable_configurations += 1;
                    }
                }

                tx.send(sharded_count_loopable_configurations).unwrap();
            });
        }

        let mut count_loopable_configurations = 0;
        for _ in 0..available_parallelism {
            count_loopable_configurations += rx.recv().unwrap();
        }

        println!(
            "We could find {count_loopable_configurations} configurations that resulted in a loop."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_day6_part2(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day6/input.txt").unwrap();

        b.iter(|| SolverImpl::solve_part2(&file));
    }
}
