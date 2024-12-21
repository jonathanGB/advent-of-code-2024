use crate::{
    solver::Solver,
    utils::{Position, generate_benchmark, pos},
};

const OUT_OF_BOUNDS_PLANT: char = '?';

#[derive(Clone, Debug)]
struct GardenPlot {
    position: Position,
    area_id: usize,
    side_up: bool,
    side_right: bool,
    side_down: bool,
    side_left: bool,
}

impl GardenPlot {
    fn num_sides(&self) -> u32 {
        let mut num_sides = 0;

        if self.side_up {
            num_sides += 1;
        }
        if self.side_right {
            num_sides += 1;
        }
        if self.side_down {
            num_sides += 1;
        }
        if self.side_left {
            num_sides += 1;
        }

        num_sides
    }
}

#[derive(Debug)]
struct Area {
    garden_plot_positions: Vec<Position>,
}

impl Area {
    fn area(&self) -> u32 {
        self.garden_plot_positions.len() as u32
    }

    fn perimeter(&self, garden_plots: &Vec<Vec<Option<GardenPlot>>>) -> u32 {
        self.garden_plot_positions
            .iter()
            .map(|Position { row, col }| garden_plots[*row][*col].as_ref().unwrap().num_sides())
            .sum()
    }

    fn perimeter_based_price(&self, garden_plots: &Vec<Vec<Option<GardenPlot>>>) -> u32 {
        self.area() * self.perimeter(garden_plots)
    }
}

#[derive(Debug)]
struct Arrangement {
    areas: Vec<Area>,
    garden_plots: Vec<Vec<Option<GardenPlot>>>,
}

impl Arrangement {
    fn new(file: &str) -> Self {
        // Note that we pad the grid with an out-of-bounds layer.
        let grid_size = file.lines().next().unwrap().len() + 2;
        // Intermediate representation. Stores a garden plot plant, and whether it's been added
        // to an area yet.
        let mut plant_and_part_of_existing_areas =
            vec![vec![(OUT_OF_BOUNDS_PLANT, true); grid_size]; grid_size];

        for (row, line) in file.lines().enumerate() {
            for (col, plant) in line.char_indices() {
                plant_and_part_of_existing_areas[row + 1][col + 1] = (plant, false);
            }
        }

        // Visit every garden plot to decide a new area must be defined. Build a new grid of fully built garden plots.
        // This grid is again padded with an out-of-bounds layer, represented with `None`.
        // Ignore out-of-bounds plots.
        let mut areas = Vec::new();
        let mut garden_plots = vec![vec![None; grid_size]; grid_size];
        for row in 1..grid_size - 1 {
            for col in 1..grid_size - 1 {
                let (plant, part_of_existing_area) = plant_and_part_of_existing_areas[row][col];
                if part_of_existing_area {
                    continue;
                }

                let area_garden_plots = Self::define_new_area(
                    plant,
                    areas.len(),
                    pos!(row, col),
                    &mut plant_and_part_of_existing_areas,
                );

                areas.push(Area {
                    garden_plot_positions: area_garden_plots
                        .iter()
                        .map(|garden_plot| garden_plot.position)
                        .collect(),
                });

                for area_garden_plot in area_garden_plots {
                    let Position { row, col } = area_garden_plot.position;
                    garden_plots[row][col] = Some(area_garden_plot);
                }
            }
        }

        Self {
            areas,
            garden_plots,
        }
    }

    fn define_new_area(
        plant: char,
        area_id: usize,
        position: Position,
        plant_and_part_of_existing_areas: &mut Vec<Vec<(char, bool)>>,
    ) -> Vec<GardenPlot> {
        let mut plots_to_explore = vec![position];
        let mut garden_plots = Vec::new();

        // Iteratively finds all surrounding plots with the same plant. The `part_of_existing_area`
        // tracks whether the plot has already been visited.
        while let Some(plot_to_explore) = plots_to_explore.pop() {
            let Position { row, col } = plot_to_explore;
            let (_, part_of_existing_area) = &mut plant_and_part_of_existing_areas[row][col];
            if *part_of_existing_area {
                continue;
            }
            *part_of_existing_area = true;

            let mut side_up = true;
            let mut side_right = true;
            let mut side_down = true;
            let mut side_left = true;
            for neighbouring_plot in plot_to_explore.surroundings() {
                let Position {
                    row: neighbour_row,
                    col: neighbour_col,
                } = neighbouring_plot;
                let (neighbour_plant, neighbour_plant_part_of_existing_area) =
                    plant_and_part_of_existing_areas[neighbour_row][neighbour_col];
                if neighbour_plant != plant {
                    continue;
                }

                if !neighbour_plant_part_of_existing_area {
                    plots_to_explore.push(neighbouring_plot);
                }

                if neighbouring_plot == plot_to_explore.up(1) {
                    side_up = false;
                } else if neighbouring_plot == plot_to_explore.right(1) {
                    side_right = false;
                } else if neighbouring_plot == plot_to_explore.down(1) {
                    side_down = false;
                } else if neighbouring_plot == plot_to_explore.left(1) {
                    side_left = false;
                } else {
                    unreachable!()
                }
            }

            garden_plots.push(GardenPlot {
                position: plot_to_explore,
                area_id,
                side_up,
                side_right,
                side_down,
                side_left,
            });
        }

        garden_plots
    }

    fn perimeter_based_price(&self) -> u32 {
        self.areas
            .iter()
            .map(|area| area.perimeter_based_price(&self.garden_plots))
            .sum()
    }

    fn num_of_sides_based_price(&self) -> u32 {
        let mut num_sides_per_area = vec![0; self.areas.len()];

        // Visit every garden plot left to right, row by row, whilst ignoring out-of-bounds plots.
        // Throughout this process, we will keep track of new sides up and down that we visit.
        for row in 1..self.garden_plots.len() - 1 {
            let mut visiting_up_area_id = None;
            let mut visiting_down_area_id = None;

            for col in 1..self.garden_plots.len() - 1 {
                let current_garden_plot = self.garden_plots[row][col].as_ref().unwrap();

                match (visiting_up_area_id, current_garden_plot.side_up) {
                    // If the next plot has a side up but is part of the same area as the previous plot,
                    // nothing to do.
                    (Some(visiting_up_area_id), true)
                        if visiting_up_area_id == current_garden_plot.area_id => {}
                    // Else if the next plot has a side up, then we found a new side up. Keep track of it.
                    (_, true) => {
                        num_sides_per_area[current_garden_plot.area_id] += 1;
                        visiting_up_area_id = Some(current_garden_plot.area_id);
                    }
                    // Else if the next plot has no side up, stop tracking.
                    (_, false) => visiting_up_area_id = None,
                }

                match (visiting_down_area_id, current_garden_plot.side_down) {
                    // If the next plot has a side down but is part of the same area as the previous plot,
                    // nothing to do.
                    (Some(visiting_down_area_id), true)
                        if visiting_down_area_id == current_garden_plot.area_id => {}
                    // Else if the next plot has a side down, then we found a new side down. Keep track of it.
                    (_, true) => {
                        num_sides_per_area[current_garden_plot.area_id] += 1;
                        visiting_down_area_id = Some(current_garden_plot.area_id);
                    }
                    // Else if the next plot has no side down, stop tracking.
                    (_, false) => visiting_down_area_id = None,
                }
            }
        }

        // Visit every garden plot top to bottom, column by column, whilst ignoring out-of-bounds plots.
        // Throughout this process, we will keep track of new sides right and left that we visit.
        for col in 1..self.garden_plots.len() - 1 {
            let mut visiting_right_area_id = None;
            let mut visiting_left_area_id = None;

            for row in 1..self.garden_plots.len() - 1 {
                let current_garden_plot = self.garden_plots[row][col].as_ref().unwrap();

                match (visiting_right_area_id, current_garden_plot.side_right) {
                    // If the next plot has a side right but is part of the same area as the previous plot,
                    // nothing to do.
                    (Some(visiting_right_area_id), true)
                        if visiting_right_area_id == current_garden_plot.area_id => {}
                    // Else if the next plot has a side right, then we found a new side right. Keep track of it.
                    (_, true) => {
                        num_sides_per_area[current_garden_plot.area_id] += 1;
                        visiting_right_area_id = Some(current_garden_plot.area_id);
                    }
                    // Else if the next plot has no side right, stop tracking.
                    (_, false) => visiting_right_area_id = None,
                }

                match (visiting_left_area_id, current_garden_plot.side_left) {
                    // If the next plot has a side left but is part of the same area as the previous plot,
                    // nothing to do.
                    (Some(visiting_left_area_id), true)
                        if visiting_left_area_id == current_garden_plot.area_id => {}
                    // Else if the next plot has a side left, then we found a new side left. Keep track of it.
                    (_, true) => {
                        num_sides_per_area[current_garden_plot.area_id] += 1;
                        visiting_left_area_id = Some(current_garden_plot.area_id);
                    }
                    // Else if the next plot has no side left, stop tracking.
                    (_, false) => visiting_left_area_id = None,
                }
            }
        }

        self.areas
            .iter()
            .zip(num_sides_per_area)
            .map(|(area, num_sides)| area.area() * num_sides)
            .sum()
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let arrangement = Arrangement::new(file);
        println!(
            "The price for fencing this arrangement is {}",
            arrangement.perimeter_based_price()
        );
    }

    fn solve_part2(file: &str) {
        let arrangement = Arrangement::new(file);
        println!(
            "The price for fencing this arrangement is {}",
            arrangement.num_of_sides_based_price()
        );
    }
}

generate_benchmark!(day12);
