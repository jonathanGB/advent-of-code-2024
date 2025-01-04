use std::{cmp::Reverse, collections::BinaryHeap, usize};

use crate::{
    solver::Solver,
    utils::{Direction, Position, generate_benchmark, pos},
};
use hashbrown::HashSet;
use itertools::Itertools;

const COST_MOVE: usize = 1;
const COST_TURN: usize = 1000;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
    _Visited,
}

impl Tile {
    fn is_wall(&self) -> bool {
        *self == Self::Wall
    }

    fn is_start(&self) -> bool {
        *self == Self::Start
    }

    fn is_end(&self) -> bool {
        *self == Self::End
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::_Visited => 'O',
        }
    }
}

struct MinScoresPerTileDirection(Vec<Vec<MinScorePerDirection>>);

impl MinScoresPerTileDirection {
    fn new(maze: &Vec<Vec<Tile>>) -> Self {
        Self(vec![
            vec![MinScorePerDirection::default(); maze[0].len()];
            maze.len()
        ])
    }

    fn update_min_score_if_not_greater(&mut self, action: &Action) -> bool {
        if self.0[action.position.row][action.position.col].min_score(action.direction)
            < action.score
        {
            false
        } else {
            *self.0[action.position.row][action.position.col].min_score_mut(action.direction) =
                action.score;
            true
        }
    }
}

#[derive(Clone)]
struct MinScorePerDirection {
    up: usize,
    right: usize,
    down: usize,
    left: usize,
}

impl MinScorePerDirection {
    fn min_score(&self, direction: Direction) -> usize {
        match direction {
            Direction::Up => self.up,
            Direction::Right => self.right,
            Direction::Down => self.down,
            Direction::Left => self.left,
        }
    }

    fn min_score_mut(&mut self, direction: Direction) -> &mut usize {
        match direction {
            Direction::Up => &mut self.up,
            Direction::Right => &mut self.right,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
        }
    }
}

impl Default for MinScorePerDirection {
    fn default() -> Self {
        Self {
            up: usize::MAX,
            right: usize::MAX,
            down: usize::MAX,
            left: usize::MAX,
        }
    }
}

struct ActionHistory {
    position: Position,
    previous_action_history_index: Option<usize>,
}

#[derive(Debug)]
struct BestPaths {
    score: usize,
    unique_tiles: HashSet<Position>,
}

#[derive(Clone, Debug)]
struct Action {
    position: Position,
    direction: Direction,
    score: usize,
    history_index: usize,
    previous_action_history_index: Option<usize>,
}

// We order Actions strictly based on the score. This is necessary
// to pop Actions from the min-heap of Actions, so that we always
// work with the Actions with the best score.
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Action {}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

struct ReindeerMaze {
    maze: Vec<Vec<Tile>>,
    start_position: Position,
    end_position: Position,
}

impl ReindeerMaze {
    fn new(file: &str) -> Self {
        let maze: Vec<Vec<_>> = file
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        let mut start_position = None;
        let mut end_position = None;
        for (i, row) in maze.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if tile.is_start() {
                    start_position = Some(pos!(i, j));
                } else if tile.is_end() {
                    end_position = Some(pos!(i, j));
                }
            }
        }

        let start_position = start_position.unwrap();
        let end_position = end_position.unwrap();

        Self {
            maze,
            start_position,
            end_position,
        }
    }

    fn _display_map(&self) -> String {
        self.maze
            .iter()
            .map(|tiles| {
                tiles
                    .iter()
                    .map(|tile| char::from(tile))
                    .collect::<String>()
            })
            .join("\n")
    }

    fn _display_map_with_visited_tiles(&self, visited_tiles: &HashSet<Position>) -> String {
        let mut maze = self.maze.clone();
        for (i, row) in maze.iter_mut().enumerate() {
            for (j, tile) in row.iter_mut().enumerate() {
                if visited_tiles.contains(&pos!(i, j)) {
                    *tile = Tile::_Visited;
                }
            }
        }

        maze.iter()
            .map(|tiles| {
                tiles
                    .iter()
                    .map(|tile| char::from(tile))
                    .collect::<String>()
            })
            .join("\n")
    }

    fn is_end_action(&self, action: &Action) -> bool {
        self.end_position == action.position
    }

    fn at(&self, position: Position) -> Tile {
        self.maze[position.row][position.col]
    }

    fn record_best_paths_unique_tiles(
        end_action: &Action,
        actions_history: &[ActionHistory],
        best_paths_unique_tiles: &mut HashSet<Position>,
    ) {
        best_paths_unique_tiles.insert(end_action.position);

        // Walk backwards through the given best path, until we reach the start position.
        let mut previous_action_history_index = end_action.previous_action_history_index;
        while let Some(index) = previous_action_history_index {
            let previous_action_history = &actions_history[index];
            best_paths_unique_tiles.insert(previous_action_history.position);
            previous_action_history_index = previous_action_history.previous_action_history_index;
        }
    }

    fn find_best_paths(&self) -> BestPaths {
        // Min-heap of potential actions, which will prioritize fetching the action with the lowest score.
        // If we repeat this process, we can guarantee via Dijkstra to generate the shortest path.
        let mut potential_actions = BinaryHeap::new();
        // Keep track of all actions that are generated throughout this search. Each item has a pointer
        // to the previous action that led to the current action.
        // This is crucial to generate the path taken once a best path to the end is found.
        let mut actions_history = Vec::new();
        // Records all unique tiles visited across all known best paths. Each of these best paths will share
        // the same `best_paths_score`.
        let mut best_paths_unique_tiles = HashSet::new();
        let mut best_paths_score = None;
        // Crucial pruning mechanism: we keep track for each tile the minimum score that has reached this
        // point for each direction. That way, if we make it to a tile in a given direction that already
        // has been visited with a lower score, then necessarily the given path is not worth pursuing.
        let mut min_scores_per_tile_direction = MinScoresPerTileDirection::new(&self.maze);

        // We start with the start tile, which we are told we are facing East (right).
        let start_action = Action {
            position: self.start_position,
            direction: Direction::Right,
            score: 0, // Start position incurred no cost so far.
            history_index: 0,
            previous_action_history_index: None, // Start action has no previous action.
        };
        actions_history.push(ActionHistory {
            position: start_action.position,
            previous_action_history_index: start_action.previous_action_history_index,
        });
        potential_actions.push(Reverse(start_action));

        // Iterative Dijkstra.
        while let Some(Reverse(action)) = potential_actions.pop() {
            // If we have found a best path, and the current action has a score that is larger than
            // that best score, than we can stop completely. That path will surely not be a best path,
            // and all remaining actions fetched from this min-heap will not have a smaller score,
            // so there is no point in pursuing.
            if best_paths_score.unwrap_or(usize::MAX) < action.score {
                break;
            }

            // If the action has a score that is larger than what is historically recorded for that tile
            // and direction, then that path is not worth pursuing.
            if !min_scores_per_tile_direction.update_min_score_if_not_greater(&action) {
                continue;
            }

            if self.is_end_action(&action) {
                if best_paths_score.is_none() {
                    best_paths_score = Some(action.score);
                }

                if best_paths_score.unwrap() > action.score {
                    unreachable!("Dijkstra guarantees finding the shortest path first");
                }

                Self::record_best_paths_unique_tiles(
                    &action,
                    &actions_history,
                    &mut best_paths_unique_tiles,
                );

                // We don't search further on this path if we have reached the end.
                continue;
            }

            // Try to move forward, but only do so if we are not facing a wall.
            let forward_position = action.position.go(action.direction);
            let forward_tile = self.at(forward_position);
            if !forward_tile.is_wall() {
                let forward_action = Action {
                    position: forward_position,
                    direction: action.direction,
                    score: action.score + COST_MOVE,
                    history_index: actions_history.len(),
                    previous_action_history_index: Some(action.history_index),
                };

                // Crucial pruning: don't explore the path forward if the score of that path is
                // higher than what is recorded historically.
                if min_scores_per_tile_direction.update_min_score_if_not_greater(&forward_action) {
                    actions_history.push(ActionHistory {
                        position: forward_action.position,
                        previous_action_history_index: forward_action.previous_action_history_index,
                    });
                    potential_actions.push(Reverse(forward_action));
                }
            }

            for turn_direction in [
                action.direction.turn_clockwise(),
                action.direction.turn_counter_clockwise(),
            ] {
                // Try to turn, but only do so if moving forward after the turn is not
                // facing a wall. We can definitely not find a best path in that case.
                if !self.at(action.position.go(turn_direction)).is_wall() {
                    let turn_action = Action {
                        position: action.position,
                        direction: turn_direction,
                        score: action.score + COST_TURN,
                        history_index: actions_history.len(),
                        previous_action_history_index: Some(action.history_index),
                    };

                    // Crucial pruning: don't explore the turn if the score of that path is
                    // higher than what is recorded historically.
                    if min_scores_per_tile_direction.update_min_score_if_not_greater(&turn_action) {
                        actions_history.push(ActionHistory {
                            position: turn_action.position,
                            previous_action_history_index: turn_action
                                .previous_action_history_index,
                        });
                        potential_actions.push(Reverse(turn_action));
                    }
                }
            }
        }

        return BestPaths {
            score: best_paths_score.expect("A best path should have been found"),
            unique_tiles: best_paths_unique_tiles,
        };
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let reindeer_maze = ReindeerMaze::new(file);
        println!("Lowest score is: {}", reindeer_maze.find_best_paths().score);
    }

    fn solve_part2(file: &str) {
        let reindeer_maze = ReindeerMaze::new(file);
        println!(
            "Number of unique tiles on best paths is is: {}",
            reindeer_maze.find_best_paths().unique_tiles.len()
        );
    }
}

generate_benchmark!(day16);
