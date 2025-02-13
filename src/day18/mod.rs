use std::collections::VecDeque;

use itertools::Itertools;

use crate::{
    solver::Solver,
    utils::{Position, generate_benchmark, pos},
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Safe,
    Corrupted,
    Outside,
}

impl Tile {
    fn is_safe(&self) -> bool {
        *self == Self::Safe
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Safe => '.',
            Tile::Corrupted => '#',
            Tile::Outside => ' ',
        }
    }
}

struct MemorySpace {
    // Note that the grid is padded with outside tiles on the side.
    grid: Vec<Vec<Tile>>,
    start: Position,
    exit: Position,
    // These positions are not corrupting the grid yet -- unless
    // they are added in part 2, one by one.
    remaining_corrupted_bytes: Vec<Position>,
}

impl MemorySpace {
    fn new(file: &str) -> Self {
        let mut lines = file.lines();
        let grid_size = lines.next().unwrap().parse().unwrap();

        // +2 to add outside rows/columns.
        let mut grid = vec![Vec::new(); grid_size + 2];
        grid[0].extend(std::iter::repeat_n(Tile::Outside, grid_size + 2));
        grid.last_mut()
            .unwrap()
            .extend(std::iter::repeat_n(Tile::Outside, grid_size + 2));

        for row in 1..=grid_size {
            // Outside left column.
            grid[row].push(Tile::Outside);

            // Main grid is by default safe.
            grid[row].extend(std::iter::repeat_n(Tile::Safe, grid_size));

            // Outside right column.
            grid[row].push(Tile::Outside);
        }

        let num_bytes = lines.next().unwrap().parse().unwrap();
        for _ in 0..num_bytes {
            let line = lines.next().unwrap();
            let (col, row) = line.split_once(',').unwrap();
            let (row, col) = (
                // +1 to include the outside padding.
                row.parse::<usize>().unwrap() + 1,
                col.parse::<usize>().unwrap() + 1,
            );

            grid[row][col] = Tile::Corrupted;
        }

        let remaining_corrupted_bytes = lines
            .map(|remaining_line| {
                let (col, row) = remaining_line.split_once(',').unwrap();
                let (row, col) = (
                    // +1 to include the outside padding.
                    row.parse::<usize>().unwrap() + 1,
                    col.parse::<usize>().unwrap() + 1,
                );
                pos!(row, col)
            })
            .collect();

        let start = pos!(1, 1);
        let exit = pos!(grid_size, grid_size);

        Self {
            grid,
            start,
            exit,
            remaining_corrupted_bytes,
        }
    }

    fn _display_map(&self) -> String {
        self.grid
            .iter()
            .map(|tiles| {
                tiles
                    .iter()
                    .map(|tile| char::from(tile))
                    .collect::<String>()
            })
            .join("\n")
    }

    fn is_exit(&self, position: Position) -> bool {
        self.exit == position
    }

    fn find_shortest_exit_path_len(&self) -> Option<u64> {
        let mut tiles_to_explore = VecDeque::from([(self.start, 0)]);
        let mut visited_tiles = vec![vec![false; self.grid.len()]; self.grid.len()];

        // Iterative BFS.
        while let Some((position, steps)) = tiles_to_explore.pop_front() {
            if self.is_exit(position) {
                return Some(steps);
            }

            // Crucial pruning: prevent exploring tiles that have already been visited.
            if visited_tiles[position.row][position.col] {
                continue;
            } else {
                visited_tiles[position.row][position.col] = true;
            }

            for neighbour in position.surroundings() {
                if !self.grid[neighbour.row][neighbour.col].is_safe() {
                    continue;
                }

                if visited_tiles[neighbour.row][neighbour.col] {
                    continue;
                }

                tiles_to_explore.push_back((neighbour, steps + 1));
            }
        }

        None
    }

    // Returns the normalized position (i.e. ignoring outside padding) of the byte
    // that partitions the start and exit tiles (i.e. cannot be reached).
    fn find_first_partition_byte(&mut self) -> Position {
        // We effectively use binary search to find the corrupt byte that partitions the start
        // and exit tiles. Contrarily to a normal binary search, we are not searching for an entry,
        // but rather the boundary between entries at which point we go from a non-partitioned space to a
        // partitioned space. Therefore, we will always reach the point where the `lo` index
        // equals the `hi` index.
        //
        // Another peculiarity is that after each check, we must update the set of corrupted tiles
        // in the grid.
        let mut lo = 0;
        let mut hi = self.remaining_corrupted_bytes.len() - 1;
        let mut mi = (lo + hi) / 2;

        // Start by setting the first half of remaining bytes (including `mi`) as corrupted on the grid.
        for i in lo..=mi {
            let Position { row, col } = self.remaining_corrupted_bytes[i];
            self.grid[row][col] = Tile::Corrupted;
        }

        loop {
            match self.find_shortest_exit_path_len() {
                // If setting all remaining bytes up to `lo|hi` resolves a shortest exit path,
                // then we have found the partition point to be the following byte.
                Some(_) if lo == hi => {
                    return self.remaining_corrupted_bytes[lo + 1];
                }
                // We have found an exit path, so more bytes must be corrupted to partition the
                // exit space.
                Some(_) => {
                    lo = mi + 1;
                    mi = (lo + hi) / 2;

                    for i in lo..=mi {
                        let Position { row, col } = self.remaining_corrupted_bytes[i];
                        self.grid[row][col] = Tile::Corrupted;
                    }
                }
                // If setting all remaining bytes up to `lo|hi` does not resolve a shortest exit path,
                // then we have found the partition point to be this exact byte.
                None if lo == hi => {
                    return self.remaining_corrupted_bytes[lo];
                }
                // We have not found an exit path, so fewer bytes must be corrupted to partition the
                // exit space.
                None => {
                    hi = mi - 1;
                    mi = (lo + hi) / 2;

                    for i in mi + 1..=hi + 1 {
                        let Position { row, col } = self.remaining_corrupted_bytes[i];
                        self.grid[row][col] = Tile::Safe;
                    }
                }
            }
        }
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let memory_space = MemorySpace::new(file);

        println!(
            "Short exit path length: {}",
            memory_space
                .find_shortest_exit_path_len()
                .expect("should find shortest path")
        );
    }

    fn solve_part2(file: &str) {
        let mut memory_space = MemorySpace::new(file);

        println!(
            "First byte that partitions the start and exit: {:?}",
            memory_space.find_first_partition_byte()
        );
    }
}

generate_benchmark!(day18);
