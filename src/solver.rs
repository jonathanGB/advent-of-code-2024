use crate::args::Part;

pub trait Solver {
    fn solve(part: Part, file: &str) {
        match part {
            Part::Part1 => Self::solve_part1(file),
            Part::Part2 => Self::solve_part2(file),
        }
    }

    fn solve_part1(file: &str);
    fn solve_part2(file: &str);
}
