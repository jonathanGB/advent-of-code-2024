use crate::args::Part;

pub trait Solver {
    fn solve(part: Part) {
        match part {
            Part::Part1 => Self::solve_part1(),
            Part::Part2 => Self::solve_part2(),
        }
    }

    fn solve_part1();
    fn solve_part2();
}
