use crate::solver::Solver;

pub struct SolverImpl {}

impl SolverImpl {
    fn is_safe_level(levels: &[i32]) -> bool {
        levels
            .windows(2)
            .all(|window| (1..=3).contains(&(window[1] - window[0])))
            || levels
                .windows(2)
                .all(|window| (-3..=-1).contains(&(window[1] - window[0])))
    }
}

impl Solver for SolverImpl {
    fn solve_part1(file: String) {
        let mut num_safe_reports = 0;

        for line in file.lines() {
            let levels: Vec<_> = line
                .split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            if Self::is_safe_level(&levels) {
                num_safe_reports += 1;
            }
        }

        println!("Number of safe reports: {num_safe_reports}");
    }

    fn solve_part2(file: String) {
        let mut num_safe_reports = 0;

        for line in file.lines() {
            let levels: Vec<_> = line
                .split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            for i in 0..levels.len() {
                // Create a copy of the original `levels`, with the ith level removed.
                // Note that already safe reports will still be safe if we remove the first level,
                // hence why we do not need to first check whether a full report is safe before
                // moving on to spliced report combinations.
                let spliced_levels = [&levels[..i], &levels[i + 1..]].concat();
                if Self::is_safe_level(&spliced_levels) {
                    num_safe_reports += 1;
                    break;
                }
            }
        }

        println!("Number of safe reports: {num_safe_reports}");
    }
}
