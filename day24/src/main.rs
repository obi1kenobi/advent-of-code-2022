use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    env, fs,
};

use geometry::{Direction, Vector2D};
#[allow(unused_imports)]
use itertools::Itertools;

mod geometry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reversed_args: Vec<_> = args.iter().map(|x| x.as_str()).rev().collect();

    reversed_args
        .pop()
        .expect("Expected the executable name to be the first argument, but was missing");

    let part = reversed_args.pop().expect("part number");
    let input_file = reversed_args.pop().expect("input file");
    let content = fs::read_to_string(input_file).unwrap();

    let input_data: Vec<&str> = content.trim_end().split('\n').map(|x| x.trim()).collect();

    match part {
        "1" => {
            let result = solve_part1(&input_data);
            println!("{result}");
        }
        "2" => {
            let result = solve_part2(&input_data);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blizzard {
    initial_position: Vector2D,
    direction: Direction,
    cycle_length: usize,
}

impl Blizzard {
    fn location_at_time_step(&self, time_step: usize) -> Vector2D {
        match self.direction {
            Direction::Up | Direction::Down => {
                let delta = match self.direction {
                    Direction::Up => -1,
                    Direction::Down => 1,
                    _ => unreachable!(),
                };
                let dx = ((self.cycle_length.checked_add_signed(delta).unwrap()) * time_step)
                    % self.cycle_length;
                let base_x = (self.initial_position.x - 1) as usize;

                let final_x = (base_x + dx) % self.cycle_length + 1;
                Vector2D::new(final_x as i64, self.initial_position.y)
            }
            Direction::Left | Direction::Right => {
                let delta = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                    _ => unreachable!(),
                };
                let dy = ((self.cycle_length.checked_add_signed(delta).unwrap()) * time_step)
                    % self.cycle_length;
                let base_y = (self.initial_position.y - 1) as usize;

                let final_y = (base_y + dy) % self.cycle_length + 1;
                Vector2D::new(self.initial_position.x, final_y as i64)
            }
        }
    }
}

struct BlizzardMap {
    start: Vector2D,
    end: Vector2D,
    blizzards_by_row: BTreeMap<i64, Vec<Blizzard>>,
    blizzards_by_col: BTreeMap<i64, Vec<Blizzard>>,
    wall_x: i64,
    wall_y: i64,
}

impl BlizzardMap {
    fn new(data: &[&str]) -> Self {
        let wall_x = (data.len() - 1) as i64;
        let wall_y = (data[0].len() - 1) as i64;

        let start = data[0]
            .chars()
            .enumerate()
            .filter_map(|(i, tile)| match tile {
                '.' => Some(Vector2D::new(0, i as i64)),
                '#' => None,
                _ => unreachable!(),
            })
            .at_most_one()
            .expect("no more than one element")
            .expect("exactly one element");
        let end = data
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .filter_map(|(i, tile)| match tile {
                '.' => Some(Vector2D::new(wall_x, i as i64)),
                '#' => None,
                _ => unreachable!(),
            })
            .at_most_one()
            .expect("no more than one element")
            .expect("exactly one element");

        let mut blizzards_by_row: BTreeMap<_, Vec<_>> = Default::default();
        let mut blizzards_by_col: BTreeMap<_, Vec<_>> = Default::default();

        for (i, row) in data.iter().enumerate() {
            for (j, tile) in row.chars().enumerate() {
                let position = Vector2D::new(i as i64, j as i64);
                let horizontal_cycle_length = data[0].len() - 2;
                let vertical_cycle_length = data.len() - 2;
                let maybe_blizzard = match tile {
                    '.' | '#' => None,
                    '>' => Some(Blizzard {
                        initial_position: position,
                        direction: Direction::Right,
                        cycle_length: horizontal_cycle_length,
                    }),
                    '<' => Some(Blizzard {
                        initial_position: position,
                        direction: Direction::Left,
                        cycle_length: horizontal_cycle_length,
                    }),
                    '^' => Some(Blizzard {
                        initial_position: position,
                        direction: Direction::Up,
                        cycle_length: vertical_cycle_length,
                    }),
                    'v' => Some(Blizzard {
                        initial_position: position,
                        direction: Direction::Down,
                        cycle_length: vertical_cycle_length,
                    }),
                    _ => unreachable!("{i} {j} {tile}"),
                };

                if let Some(blizzard) = maybe_blizzard {
                    match blizzard.direction {
                        Direction::Up | Direction::Down => {
                            blizzards_by_col.entry(j as i64).or_default().push(blizzard);
                        }
                        Direction::Left | Direction::Right => {
                            blizzards_by_row.entry(i as i64).or_default().push(blizzard);
                        }
                    }
                }
            }
        }

        Self {
            start,
            end,
            blizzards_by_row,
            blizzards_by_col,
            wall_x,
            wall_y,
        }
    }

    fn tile_available_at_time_step(&self, tile: Vector2D, time_step: usize) -> bool {
        if tile.x < 0 || tile.y < 0 || tile.x > self.wall_x || tile.y > self.wall_y {
            // Spilled off the map, perhaps through the start or end tiles.
            return false;
        }

        if tile.y == 0 || tile.y == self.wall_y {
            // Hit the side walls.
            return false;
        }

        if tile.x == 0 {
            // Either hit a wall, or came back to the start.
            return tile == self.start;
        }

        if tile.x == self.wall_x {
            // Either hit a wall, or reached the end.
            return tile == self.end;
        }

        let mut blizzards_to_check = self
            .blizzards_by_row
            .get(&tile.x)
            .map(|x| x.as_slice())
            .unwrap_or_default()
            .iter()
            .chain(
                self.blizzards_by_col
                    .get(&tile.y)
                    .map(|x| x.as_slice())
                    .unwrap_or_default(),
            );
        !blizzards_to_check.any(|blizzard| blizzard.location_at_time_step(time_step) == tile)
    }
}

fn solve_part1(data: &[&str]) -> usize {
    let blizzard_map = BlizzardMap::new(data);

    let start = (0, blizzard_map.start);
    let mut visited: BTreeSet<(usize, Vector2D)> = [start].into_iter().collect();
    let mut queue: VecDeque<_> = [start].into_iter().collect();

    let moves = [
        Vector2D::new(0, 0), // stall and don't move for a turn
        Direction::Up.unit_vector(),
        Direction::Down.unit_vector(),
        Direction::Left.unit_vector(),
        Direction::Right.unit_vector(),
    ];

    let mut solution = None;

    'outer: while let Some((time, position)) = queue.pop_front() {
        let next_time = time + 1;

        for next_move in moves.iter().copied() {
            let next_position = position + next_move;
            let next_key = (next_time, next_position);
            if visited.contains(&next_key) {
                continue;
            }

            if blizzard_map.tile_available_at_time_step(next_position, next_time) {
                visited.insert(next_key);

                if next_position == blizzard_map.end {
                    solution = Some(next_key);
                    break 'outer;
                } else {
                    queue.push_back(next_key);
                }
            }
        }
    }

    solution.expect("no solution found").0
}

fn solve_part2(data: &[&str]) -> usize {
    let blizzard_map = BlizzardMap::new(data);

    let start = (0, 0, blizzard_map.start);
    let mut visited: BTreeSet<(usize, usize, Vector2D)> = [start].into_iter().collect();
    let mut queue: VecDeque<_> = [start].into_iter().collect();

    let moves = [
        Vector2D::new(0, 0), // stall and don't move for a turn
        Direction::Up.unit_vector(),
        Direction::Down.unit_vector(),
        Direction::Left.unit_vector(),
        Direction::Right.unit_vector(),
    ];

    let mut solution = None;

    'outer: while let Some((leg, time, position)) = queue.pop_front() {
        let next_time = time + 1;

        for next_move in moves.iter().copied() {
            let next_position = position + next_move;

            let next_leg = match leg {
                0 => {
                    if next_position == blizzard_map.end {
                        leg + 1
                    } else {
                        leg
                    }
                }
                1 => {
                    if next_position == blizzard_map.start {
                        leg + 1
                    } else {
                        leg
                    }
                }
                2 => {
                    if next_position == blizzard_map.end {
                        leg + 1
                    } else {
                        leg
                    }
                }
                _ => unreachable!("{leg}"),
            };

            let next_key = (next_leg, next_time, next_position);
            if visited.contains(&next_key) {
                continue;
            }

            if blizzard_map.tile_available_at_time_step(next_position, next_time) {
                visited.insert(next_key);

                if next_leg == 3 && next_position == blizzard_map.end {
                    solution = Some(next_key);
                    break 'outer;
                } else {
                    queue.push_back(next_key);
                }
            }
        }
    }

    solution.expect("no solution found").1
}
