use std::collections::HashMap;

use crate::{solver::Solver, utils::generate_benchmark};

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let (mut location_ids_a, mut location_ids_b): (Vec<_>, Vec<_>) = file
            .lines()
            .map(|line| {
                let (location_id_a, location_id_b) = line.split_once("   ").unwrap();
                (
                    location_id_a.parse::<i32>().unwrap(),
                    location_id_b.parse::<i32>().unwrap(),
                )
            })
            .unzip();
        location_ids_a.sort();
        location_ids_b.sort();

        let mut total = 0;
        for (location_id_a, location_id_b) in
            location_ids_a.into_iter().zip(location_ids_b.into_iter())
        {
            total += (location_id_b - location_id_a).abs();
        }

        println!("Total is {total}");
    }

    fn solve_part2(file: &str) {
        let (location_ids_a, location_ids_b): (Vec<_>, Vec<_>) = file
            .lines()
            .map(|line| {
                let (location_id_a, location_id_b) = line.split_once("   ").unwrap();
                (
                    location_id_a.parse::<i32>().unwrap(),
                    location_id_b.parse::<i32>().unwrap(),
                )
            })
            .unzip();
        let mut location_ids_and_count_a: HashMap<i32, i32> = HashMap::new();
        for location_id_a in location_ids_a {
            location_ids_and_count_a
                .entry(location_id_a)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut location_ids_and_count_b: HashMap<i32, i32> = HashMap::new();
        for location_id_b in location_ids_b {
            location_ids_and_count_b
                .entry(location_id_b)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut total = 0;
        for (location_id, count) in location_ids_and_count_a {
            let location_id_b_count = location_ids_and_count_b.get(&location_id).unwrap_or(&0);

            total += count * (location_id * location_id_b_count);
        }

        println!("Total is {total}");
    }
}

generate_benchmark!(day1);
