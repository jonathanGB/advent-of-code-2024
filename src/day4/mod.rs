use crate::solver::Solver;
use crate::utils::{Position, generate_benchmark};

// "MAS" is 3 characters long.
const MAS_LENGTH: usize = 3;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl From<char> for Letter {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Letter>>,
    // The grid is a square of size `size`.
    size: usize,
}

impl Grid {
    fn new(file: &str) -> Self {
        let grid: Vec<_> = file
            .lines()
            .map(|line| line.chars().map(Letter::from).collect::<Vec<_>>())
            .collect();
        let size = grid.len();
        Self { grid, size }
    }

    fn at(&self, position: Position) -> Letter {
        self.grid[position.row][position.col]
    }

    fn count_all_xmas_occurrences(&self) -> usize {
        let x_positions = self.find_all_letter_positions(Letter::X);
        let mut xmax_occurences = 0;

        for x_position in x_positions {
            // Try up.
            if x_position.row >= MAS_LENGTH
                && self.at(x_position.up(1)) == Letter::M
                && self.at(x_position.up(2)) == Letter::A
                && self.at(x_position.up(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal up-right.
            if x_position.row >= MAS_LENGTH
                && x_position.col < self.size - MAS_LENGTH
                && self.at(x_position.up(1).right(1)) == Letter::M
                && self.at(x_position.up(2).right(2)) == Letter::A
                && self.at(x_position.up(3).right(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try right.
            if x_position.col < self.size - MAS_LENGTH
                && self.at(x_position.right(1)) == Letter::M
                && self.at(x_position.right(2)) == Letter::A
                && self.at(x_position.right(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal down-right.
            if x_position.row < self.size - MAS_LENGTH
                && x_position.col < self.size - MAS_LENGTH
                && self.at(x_position.down(1).right(1)) == Letter::M
                && self.at(x_position.down(2).right(2)) == Letter::A
                && self.at(x_position.down(3).right(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try down.
            if x_position.row < self.size - MAS_LENGTH
                && self.at(x_position.down(1)) == Letter::M
                && self.at(x_position.down(2)) == Letter::A
                && self.at(x_position.down(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal down-left.
            if x_position.row < self.size - MAS_LENGTH
                && x_position.col >= MAS_LENGTH
                && self.at(x_position.down(1).left(1)) == Letter::M
                && self.at(x_position.down(2).left(2)) == Letter::A
                && self.at(x_position.down(3).left(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try left.
            if x_position.col >= MAS_LENGTH
                && self.at(x_position.left(1)) == Letter::M
                && self.at(x_position.left(2)) == Letter::A
                && self.at(x_position.left(3)) == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal up-left.
            if x_position.row >= MAS_LENGTH
                && x_position.col >= MAS_LENGTH
                && self.at(x_position.up(1).left(1)) == Letter::M
                && self.at(x_position.up(2).left(2)) == Letter::A
                && self.at(x_position.up(3).left(3)) == Letter::S
            {
                xmax_occurences += 1;
            }
        }

        xmax_occurences
    }

    fn count_all_x_mas_occurrences(&self) -> usize {
        let a_positions = self.find_all_letter_positions(Letter::A);
        let mut x_mas_occurrences = 0;

        for a_position in a_positions {
            if a_position.row == 0
                || a_position.col == self.size - 1
                || a_position.row == self.size - 1
                || a_position.col == 0
            {
                continue;
            }

            if ((self.at(a_position.up(1).left(1)) == Letter::M
                && self.at(a_position.down(1).right(1)) == Letter::S)
                || (self.at(a_position.up(1).left(1)) == Letter::S
                    && self.at(a_position.down(1).right(1)) == Letter::M))
                && ((self.at(a_position.down(1).left(1)) == Letter::M
                    && self.at(a_position.up(1).right(1)) == Letter::S)
                    || (self.at(a_position.down(1).left(1)) == Letter::S
                        && self.at(a_position.up(1).right(1)) == Letter::M))
            {
                x_mas_occurrences += 1;
            }
        }

        x_mas_occurrences
    }

    fn find_all_letter_positions(&self, letter: Letter) -> Vec<Position> {
        let mut letter_positions = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                if self.grid[row][col] == letter {
                    letter_positions.push(Position { row, col });
                }
            }
        }

        letter_positions
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let grid = Grid::new(&file);
        println!("XMAS appeared {} times.", grid.count_all_xmas_occurrences());
    }

    fn solve_part2(file: &str) {
        let grid = Grid::new(&file);
        println!(
            "X-MAS appeared {} times.",
            grid.count_all_x_mas_occurrences()
        );
    }
}

generate_benchmark!(day4);
