use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::Deref,
    str::{FromStr, Lines},
};

use crate::solver::Solver;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
struct Page(u16);

impl FromStr for Page {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let page = s.parse()?;
        Ok(Self(page))
    }
}

impl Deref for Page {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct OrderingRules {
    // Maps a Page to the set of Pages that the former must be present before.
    // I.e. 5 => {3,9} means that page 5 must be before both page 3 and page 9.
    page_and_before_pages: HashMap<Page, HashSet<Page>>,
}

impl Deref for OrderingRules {
    type Target = HashMap<Page, HashSet<Page>>;

    fn deref(&self) -> &Self::Target {
        &self.page_and_before_pages
    }
}

impl OrderingRules {
    fn new(lines: &mut Lines) -> Self {
        let mut page_and_before_pages = HashMap::<Page, HashSet<Page>>::default();

        for line in lines {
            if line.is_empty() {
                break;
            }

            let (before, after) = line.split_once('|').unwrap();
            let (before, after) = (before.parse().unwrap(), after.parse().unwrap());

            page_and_before_pages
                .entry(before)
                .or_default()
                .insert(after);
        }

        Self {
            page_and_before_pages,
        }
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: String) {
        let mut lines = file.lines();
        let ordering_rules = OrderingRules::new(&mut lines);
        let mut sum_middle_pages = 0;

        'updates: for line in lines {
            let pages: Vec<Page> = line.split(',').map(|page| page.parse().unwrap()).collect();
            let middle_page = *pages[(pages.len() - 1) / 2];

            let pages_by_position: HashMap<Page, usize> = pages
                .into_iter()
                .enumerate()
                .map(|(i, page)| (page, i))
                .collect();

            for (page, page_position) in &pages_by_position {
                // Check that the current page satisfies the ordering rules, meaning that it is
                // present before the set of `before_pages`. If there is none, then there is no
                // ordering rule for that page, so continue.
                let before_pages = match ordering_rules.get(page) {
                    Some(before_pages) => before_pages,
                    None => continue,
                };

                for before_page in before_pages {
                    // Find the position of a before page in the current update. If that position
                    // is less than the current page, the update is invalid -- ignore it. If there
                    // is no position, then that page is not present in this update, we can ignore
                    // this rule.
                    let before_page_position = match pages_by_position.get(before_page) {
                        Some(before_page_position) => before_page_position,
                        None => continue,
                    };

                    if page_position > before_page_position {
                        continue 'updates;
                    }
                }
            }

            sum_middle_pages += middle_page;
        }

        println!("The sum of valid middle pages is {sum_middle_pages}");
    }

    fn solve_part2(file: String) {
        println!("{file}");
        unimplemented!()
    }
}
