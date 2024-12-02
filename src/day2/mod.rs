use crate::solver::Solver;

pub struct SolverImpl {}

impl SolverImpl {
    fn is_safe_level(levels: &[Option<i32>]) -> bool {
        // Either the 1st or the 2nd level must be present (both cannot be absent).
        // Note that levels are only potentially ignored in part 2.
        let (mut previous_level, start_index) = match levels[0] {
            Some(level_0) => (level_0, 1),
            None => match levels[1] {
                Some(level_1) => (level_1, 2),
                None => unreachable!(),
            },
        };
        let mut report_ordering = None;
        for i in start_index..levels.len() {
            // Ignored values (i.e. None) are skipped.
            let curr_level = match levels[i] {
                Some(curr_level) => curr_level,
                None => continue,
            };

            let curr_ordering = previous_level.cmp(&curr_level);
            // Contiguous levels cannot be equal.
            if curr_ordering.is_eq() {
                return false;
            }

            match report_ordering {
                None => report_ordering = Some(curr_ordering),
                // Contiguous levels have to follow the same ordering throughout the report.
                Some(report_ordering) if report_ordering != curr_ordering => return false,
                // If we have a determined ordering and it matches the current pair ordering, nothing to do.
                Some(_) => {}
            };

            let diff = (curr_level - previous_level).abs();
            // Contiguous levels can at most have a difference of 3.
            if diff > 3 {
                return false;
            }

            previous_level = curr_level;
        }

        return true;
    }
}

impl Solver for SolverImpl {
    fn solve_part1(file: String) {
        let mut num_safe_reports = 0;

        for line in file.lines() {
            let levels: Vec<_> = line
                .split(' ')
                .map(|level| Some(level.parse::<i32>().unwrap()))
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
            let mut levels: Vec<_> = line
                .split(' ')
                .map(|level| Some(level.parse::<i32>().unwrap()))
                .collect();

            if Self::is_safe_level(&levels) {
                num_safe_reports += 1;
            } else {
                // The current report is unsafe. Try to ignore each level one after the other, until we
                // find one combination that is deemed safe.

                // This keeps track of the previous value that was ignored when checking a new report combination.
                // This is important when we need to try a new report combination, so that the previous level
                // that was ignored is replaced with its original value.
                let mut previous_ignored_value = None;
                for i in 0..levels.len() {
                    if previous_ignored_value.is_some() {
                        levels[i - 1] = previous_ignored_value;
                    }

                    previous_ignored_value = std::mem::take(&mut levels[i]);

                    if Self::is_safe_level(&levels) {
                        num_safe_reports += 1;
                        break;
                    }
                }
            }
        }

        println!("Number of safe reports: {num_safe_reports}");
    }
}
