use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::{FromStr, Lines},
};

use crate::{solver::Solver, utils::generate_benchmark};

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
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

#[derive(Default, Debug)]
struct TopologicalPage {
    // Set of pages that must be after this page
    // (i.e. outgoing in a topological graph).
    must_be_after_pages: HashSet<Page>,
    // Num of pages in the setup that must be before this page
    // (i.e. incoming in a topological graph).
    num_must_be_before_pages: usize,
}

#[derive(Debug)]
struct TopologicalPages {
    topological_pages: HashMap<Page, TopologicalPage>,
}

impl Deref for TopologicalPages {
    type Target = HashMap<Page, TopologicalPage>;

    fn deref(&self) -> &Self::Target {
        &self.topological_pages
    }
}

impl DerefMut for TopologicalPages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.topological_pages
    }
}

impl TopologicalPages {
    fn new(ordering_rules: &OrderingRules, pages_of_interest: &HashSet<Page>) -> Self {
        let mut topological_pages = HashMap::<Page, TopologicalPage>::default();

        for page_of_interest in pages_of_interest {
            // Check if the page of interest has rules regarding pages that must be present after.
            // If not, nothing to do here.
            // If so, only grab the ones that apply to this problem.
            let must_be_after_pages = match ordering_rules.get(page_of_interest) {
                Some(must_be_after_pages) => must_be_after_pages.intersection(pages_of_interest),
                None => continue,
            };

            // Update the set of outgoing nodes (i.e. pages that must be after this one).
            topological_pages
                .entry(*page_of_interest)
                .or_default()
                .must_be_after_pages
                .extend(must_be_after_pages.clone());

            // Increment the counter of incoming edge to each outgoing node.
            for must_be_after_page in must_be_after_pages {
                topological_pages
                    .entry(*must_be_after_page)
                    .or_default()
                    .num_must_be_before_pages += 1;
            }
        }

        Self { topological_pages }
    }

    // Sorts topologically `pages` based on the `ordering_rules`.
    // If `pages` was already sorted topologically, returns None.
    // Otherwise, returns the topologically sorted list of pages.
    fn sort_topologically(ordering_rules: &OrderingRules, pages: &[Page]) -> Option<Vec<Page>> {
        let mut topological_pages = Self::new(ordering_rules, &pages.iter().cloned().collect());
        let mut topologically_sorted_pages = Vec::new();

        // Topological sorting is a loop that:
        //   1. Finds the root of the sub-DAG, i.e. the node in a DAG with no incoming edges.
        //   2. Pushes the root in the topologically sorted list.
        //   3. Removes the root of the sub-DAG.
        //   4. Decrements the counter of incoming edges for each node which the root connected to.
        while !topological_pages.is_empty() {
            let topological_root_page = *topological_pages
                .iter()
                .find(|(_, topological_page)| topological_page.num_must_be_before_pages == 0)
                .unwrap()
                .0;
            topologically_sorted_pages.push(topological_root_page);
            let topological_root_page = topological_pages.remove(&topological_root_page).unwrap();

            for must_be_after_page in topological_root_page.must_be_after_pages {
                topological_pages
                    .get_mut(&must_be_after_page)
                    .unwrap()
                    .num_must_be_before_pages -= 1;
            }
        }

        if pages == topologically_sorted_pages {
            None
        } else {
            Some(topologically_sorted_pages)
        }
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
    fn solve_part1(file: &str) {
        let mut lines = file.lines();
        let ordering_rules = OrderingRules::new(&mut lines);
        let mut sum_middle_pages = 0;

        for line in lines {
            let pages: Vec<Page> = line.split(',').map(|page| page.parse().unwrap()).collect();

            if TopologicalPages::sort_topologically(&ordering_rules, &pages).is_none() {
                let middle_page = *pages[(pages.len() - 1) / 2];
                sum_middle_pages += middle_page;
            }
        }

        println!("The sum of valid middle pages is {sum_middle_pages}");
    }

    fn solve_part2(file: &str) {
        let mut lines = file.lines();
        let ordering_rules = OrderingRules::new(&mut lines);
        let mut sum_middle_pages = 0;

        for line in lines {
            let pages: Vec<Page> = line.split(',').map(|page| page.parse().unwrap()).collect();

            if let Some(topologically_sorted_pages) =
                TopologicalPages::sort_topologically(&ordering_rules, &pages)
            {
                let middle_page =
                    *topologically_sorted_pages[(topologically_sorted_pages.len() - 1) / 2];
                sum_middle_pages += middle_page;
            }
        }

        println!("The sum of valid middle pages is {sum_middle_pages}");
    }
}

generate_benchmark!(day5);
