use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::{
    solver::Solver,
    utils::{Direction, Position, generate_benchmark, pos},
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Wall,
    Box,
    LeftBox,
    RightBox,
    Robot,
    Empty,
}

impl Tile {
    fn is_wall(&self) -> bool {
        *self == Self::Wall
    }

    fn is_box(&self) -> bool {
        self.is_small_box() || self.is_left_box() || self.is_right_box()
    }

    fn is_small_box(&self) -> bool {
        *self == Self::Box
    }

    fn is_left_box(&self) -> bool {
        *self == Self::LeftBox
    }

    fn is_right_box(&self) -> bool {
        *self == Self::RightBox
    }

    fn is_robot(&self) -> bool {
        *self == Self::Robot
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::LeftBox => '[',
            Tile::RightBox => ']',
            Tile::Robot => '@',
            Tile::Empty => '.',
        }
    }
}

#[derive(Debug)]
struct Robot {
    map: Vec<Vec<Tile>>,
    position: Position,
    directions: VecDeque<Direction>,
}

impl Robot {
    fn new(file: &str, wide: bool) -> Self {
        let mut lines = file.lines();
        let mut map: Vec<Vec<_>> = Vec::new();

        // Build the map first.
        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let mut map_row = Vec::new();
            for tile in line.chars() {
                match tile {
                    '#' => {
                        map_row.push(Tile::Wall);
                        if wide {
                            map_row.push(Tile::Wall);
                        }
                    }
                    'O' if wide => {
                        map_row.push(Tile::LeftBox);
                        map_row.push(Tile::RightBox);
                    }
                    'O' => {
                        map_row.push(Tile::Box);
                    }
                    '.' => {
                        map_row.push(Tile::Empty);
                        if wide {
                            map_row.push(Tile::Empty);
                        }
                    }
                    '@' => {
                        map_row.push(Tile::Robot);
                        if wide {
                            map_row.push(Tile::Empty);
                        }
                    }
                    _ => unreachable!(),
                }
            }

            map.push(map_row);
        }

        let position = Self::find_robot(&map);

        let directions = lines
            .map(|line| line.chars().map(Direction::from))
            .flatten()
            .collect();

        Self {
            map,
            position,
            directions,
        }
    }

    fn find_robot(map: &Vec<Vec<Tile>>) -> Position {
        for (i, row) in map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if tile.is_robot() {
                    return pos!(i, j);
                }
            }
        }

        unreachable!("there must be one robot on the map")
    }

    fn consume_directions_sequence(&mut self) {
        while let Some(direction) = self.directions.pop_front() {
            self.go(direction);
        }
    }

    fn at(&self, position: Position) -> Tile {
        self.map[position.row][position.col]
    }

    fn at_mut(&mut self, position: Position) -> &mut Tile {
        &mut self.map[position.row][position.col]
    }

    fn go(&mut self, direction: Direction) {
        // Nothing to do if we are going into a wall.
        if self.at(self.position.go(direction)).is_wall() {
            return;
        }

        let going_sideways = direction.sideways();
        // Keeps track of old and new box positions. Boxes should be moved from the end
        // to the start, so as to not corrupt moves. If done properly, we guarantee that a box
        // moved will be moved into an empty tile, not onto another box.
        let mut boxes_to_move = Vec::new();
        // Prevent inspecting the same tile twice, which could lead to corruption of moves.
        let mut inspected_tiles = HashSet::new();
        // We use a deque to apply a BFS. This is important, because inspected box tiles
        // are eventually added to `boxes_to_move`, and a mismatch in order could make moving
        // boxes around corrupt wide boxes.
        let mut tiles_to_inspect = VecDeque::new();

        // If the next position is a box, we start the BFS to find all touching boxes.
        if self.at(self.position.go(direction)).is_box() {
            let next_tile = self.position.go(direction);

            tiles_to_inspect.push_back(next_tile);
            inspected_tiles.insert(next_tile);

            let attached_next_tile = if going_sideways {
                // We don't need extra handling when moving sideways, as opposed to when
                // moving up and down. Even if we are working with wide boxes, these
                // will be visited through the normal walk sideways.
                None
            } else {
                match self.at(next_tile) {
                    // Small boxes don't need extra handling, even when moving up and down.
                    // These will be visited through the normal walk.
                    Tile::Box => None,
                    // A wide left box when moving up or down requires to also visit its
                    // attached right side.
                    Tile::LeftBox => Some(next_tile.right(1)),
                    // A wide right box when moving up or down requires to also visit its
                    // attached left side.
                    Tile::RightBox => Some(next_tile.left(1)),
                    _ => unreachable!("next_tile has to be a box in this branch"),
                }
            };

            if let Some(attached_next_tile) = attached_next_tile {
                tiles_to_inspect.push_back(attached_next_tile);
                inspected_tiles.insert(attached_next_tile);
            }
        }

        // Iterative BFS which will visit all touching boxes only once.
        while let Some(tile_to_inspect) = tiles_to_inspect.pop_front() {
            let next_tile_to_inspect = tile_to_inspect.go(direction);
            // If a box touches a wall, then nothing can be moved. Stop.
            if self.at(next_tile_to_inspect).is_wall() {
                return;
            }

            // The inspected box can potentially be moved. Keep track of its movement.
            boxes_to_move.push((tile_to_inspect, next_tile_to_inspect));

            // Nothing to do if the next tile is not a box, or if the box has already
            // been inspected.
            if !self.at(next_tile_to_inspect).is_box()
                || !inspected_tiles.insert(next_tile_to_inspect)
            {
                continue;
            }

            // This next box tile has not been inspected yet, add it.
            tiles_to_inspect.push_back(next_tile_to_inspect);

            // No extra handling needed when going sideways, move on to the next tile to inspect.
            if going_sideways {
                continue;
            }

            // We are going up or down and the next tile in that direction is a box. For wide boxes, we must
            // consider inspecting its attached part, if it has not been inspected yet.
            if let Some(attached_next_tile_to_inspect) = match self.at(next_tile_to_inspect) {
                Tile::Box => None,
                Tile::LeftBox => Some(next_tile_to_inspect.right(1)),
                Tile::RightBox => Some(next_tile_to_inspect.left(1)),
                _ => unreachable!("next_tile_to_inspect has to be a box in this branch"),
            } {
                if inspected_tiles.insert(attached_next_tile_to_inspect) {
                    tiles_to_inspect.push_back(attached_next_tile_to_inspect);
                }
            }
        }

        // We are done with BFS of touching boxes. The last box in the list can be moved right away. Once
        // the last one is moved, we guarantee that the second-to-last box can be moved too without corruption.
        // And so on.
        while let Some((box_old_position, box_new_position)) = boxes_to_move.pop() {
            let box_tile = self.at(box_old_position);
            *self.at_mut(box_old_position) = Tile::Empty;
            *self.at_mut(box_new_position) = box_tile;
        }

        // Finally, move the robot to its new location.
        *self.at_mut(self.position) = Tile::Empty;
        *self.at_mut(self.position.go(direction)) = Tile::Robot;
        self.position = self.position.go(direction);
    }

    fn _display_map(&self) -> String {
        self.map
            .iter()
            .map(|tiles| {
                tiles
                    .iter()
                    .map(|tile| char::from(tile))
                    .collect::<String>()
            })
            .join("\n")
    }

    fn sum_box_gps_coordinates(&self) -> usize {
        let mut sum = 0;

        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                // Only non-wide boxes and the left side of a wide box count
                // for GPS coordinates.
                if !tile.is_small_box() && !tile.is_left_box() {
                    continue;
                }

                sum += 100 * i + j;
            }
        }

        sum
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let mut robot = Robot::new(file, false);
        robot.consume_directions_sequence();
        println!(
            "Sum of the box GPS coordinates: {}",
            robot.sum_box_gps_coordinates()
        );
    }

    fn solve_part2(file: &str) {
        let mut robot = Robot::new(file, true);
        robot.consume_directions_sequence();
        println!(
            "Sum of the box GPS coordinates: {}",
            robot.sum_box_gps_coordinates()
        );
    }
}

generate_benchmark!(day15);
