use crate::solver::Solver;

// "MAS" is 3 characters long.
const MAS_LENGTH: usize = 3;

#[derive(Debug, PartialEq)]
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

struct Position {
    row: usize,
    col: usize,
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

    fn count_all_xmas_occurrences(&self) -> usize {
        let x_positions = self.find_all_letter_positions(Letter::X);
        let grid = &self.grid;
        let mut xmax_occurences = 0;

        for Position {
            row: x_row,
            col: x_col,
        } in x_positions
        {
            // Try up.
            if x_row >= MAS_LENGTH
                && grid[x_row - 1][x_col] == Letter::M
                && grid[x_row - 2][x_col] == Letter::A
                && grid[x_row - 3][x_col] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal up-right.
            if x_row >= MAS_LENGTH
                && x_col < self.size - MAS_LENGTH
                && grid[x_row - 1][x_col + 1] == Letter::M
                && grid[x_row - 2][x_col + 2] == Letter::A
                && grid[x_row - 3][x_col + 3] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try right.
            if x_col < self.size - MAS_LENGTH
                && grid[x_row][x_col + 1] == Letter::M
                && grid[x_row][x_col + 2] == Letter::A
                && grid[x_row][x_col + 3] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal down-right.
            if x_row < self.size - MAS_LENGTH
                && x_col < self.size - MAS_LENGTH
                && grid[x_row + 1][x_col + 1] == Letter::M
                && grid[x_row + 2][x_col + 2] == Letter::A
                && grid[x_row + 3][x_col + 3] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try down.
            if x_row < self.size - MAS_LENGTH
                && grid[x_row + 1][x_col] == Letter::M
                && grid[x_row + 2][x_col] == Letter::A
                && grid[x_row + 3][x_col] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal down-left.
            if x_row < self.size - MAS_LENGTH
                && x_col >= MAS_LENGTH
                && grid[x_row + 1][x_col - 1] == Letter::M
                && grid[x_row + 2][x_col - 2] == Letter::A
                && grid[x_row + 3][x_col - 3] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try left.
            if x_col >= MAS_LENGTH
                && grid[x_row][x_col - 1] == Letter::M
                && grid[x_row][x_col - 2] == Letter::A
                && grid[x_row][x_col - 3] == Letter::S
            {
                xmax_occurences += 1;
            }

            // Try diagonal up-left.
            if x_row >= MAS_LENGTH
                && x_col >= MAS_LENGTH
                && grid[x_row - 1][x_col - 1] == Letter::M
                && grid[x_row - 2][x_col - 2] == Letter::A
                && grid[x_row - 3][x_col - 3] == Letter::S
            {
                xmax_occurences += 1;
            }
        }

        xmax_occurences
    }

    fn count_all_x_mas_occurrences(&self) -> usize {
        let a_positions = self.find_all_letter_positions(Letter::A);
        let grid = &self.grid;
        let mut x_mas_occurrences = 0;

        for Position {
            row: a_row,
            col: a_col,
        } in a_positions
        {
            if a_row == 0 || a_col == self.size - 1 || a_row == self.size - 1 || a_col == 0 {
                continue;
            }

            if ((grid[a_row - 1][a_col - 1] == Letter::M
                && grid[a_row + 1][a_col + 1] == Letter::S)
                || (grid[a_row - 1][a_col - 1] == Letter::S
                    && grid[a_row + 1][a_col + 1] == Letter::M))
                && ((grid[a_row + 1][a_col - 1] == Letter::M
                    && grid[a_row - 1][a_col + 1] == Letter::S)
                    || (grid[a_row + 1][a_col - 1] == Letter::S
                        && grid[a_row - 1][a_col + 1] == Letter::M))
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
    fn solve_part1(file: String) {
        let grid = Grid::new(&file);
        println!("XMAS appeared {} times.", grid.count_all_xmas_occurrences());
    }

    fn solve_part2(file: String) {
        let grid = Grid::new(&file);
        println!(
            "X-MAS appeared {} times.",
            grid.count_all_x_mas_occurrences()
        );
    }
}
