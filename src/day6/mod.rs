use crate::solver::Solver;

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

#[derive(Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    // Moves the guard one tile following its patrol protocol.
    // That is, try to move one tile into the current direction. If the new tile is obstructed, rotate to the right,
    // and try in that new direction. Panics if the tile to the right is obstructed.
    // Returns the new position if it is inbounds. If the new position is outside (or if the starting
    // position was already outside), returns None.
    fn patrol(&mut self, lab: &Vec<Vec<Tile>>) -> Option<Position> {
        let Position { row, col } = self.position;
        if lab[row][col].is_outside() {
            return None;
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
            None
        } else if !new_tile.is_obstructed() {
            self.position = new_position;
            Some(new_position)
        } else if alternative_new_tile.is_outside() {
            None
        } else if alternative_new_tile.is_obstructed() {
            unreachable!()
        } else {
            self.position = alternative_new_position;
            self.direction = alternative_new_direction;
            Some(alternative_new_position)
        }
    }
}

#[derive(Debug)]
struct LabSimulation {
    // Note that the lab is padded all around with "outside" tiles.
    lab: Vec<Vec<Tile>>,
    guard: Guard,
    num_unique_visited_tiles: usize,
}

impl LabSimulation {
    fn new(file: String) -> Self {
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
            num_unique_visited_tiles: 1,
        }
    }

    // Runs the guard patrol, and returns the number of unique tiles
    // visited by the guard after their exit from the lab.
    fn run_guard_patrol(mut self) -> usize {
        while let Some(Position { row, col }) = self.guard.patrol(&self.lab) {
            if self.lab[row][col].is_unvisited() {
                self.num_unique_visited_tiles += 1;
                self.lab[row][col] = Tile::Visited;
            }
        }

        self.num_unique_visited_tiles
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: String) {
        let lab_simulation = LabSimulation::new(file);
        let num_unique_visited_tiles = lab_simulation.run_guard_patrol();
        println!(
            "The guard visited {} unique tiles.",
            num_unique_visited_tiles
        );
    }

    fn solve_part2(file: String) {
        println!("{file}");
        unimplemented!()
    }
}
