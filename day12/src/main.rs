use std::{collections::VecDeque, env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reversed_args: Vec<_> = args.iter().map(|x| x.as_str()).rev().collect();

    reversed_args
        .pop()
        .expect("Expected the executable name to be the first argument, but was missing");

    let part = reversed_args.pop().expect("part number");
    let input_file = reversed_args.pop().expect("input file");
    let content = fs::read_to_string(input_file).unwrap();

    let input_data: Vec<Vec<char>> = content
        .trim_end()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

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

fn locate_char(data: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    data.iter().enumerate().find_map(|(i, row)| {
        row.iter()
            .find_position(|c| **c == target)
            .map(|(j, _)| (i, j))
    })
}

fn offset_coords(
    x: usize,
    y: usize,
    dx: i64,
    dy: i64,
    max_x: usize,
    max_y: usize,
) -> Option<(usize, usize)> {
    let new_x = (x as i64) - dx;
    let new_y = (y as i64) - dy;

    if new_x >= 0 && new_y >= 0 {
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        if new_x < max_x && new_y < max_y {
            return Some((new_x, new_y));
        }
    }

    None
}

fn solve_part1(data: &[Vec<char>]) -> u64 {
    let (start_i, start_j) = locate_char(data, 'S').expect("no start");
    let (end_i, end_j) = locate_char(data, 'E').expect("no end");

    let heights = data
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match *c {
                    'S' => 0,
                    'E' => ('z' as u32) - ('a' as u32),
                    c => (c as u32) - ('a' as u32),
                })
                .collect_vec()
        })
        .collect_vec();

    let max_i = heights.len();
    let max_j = heights[0].len();

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    let distance_row = vec![u64::MAX; max_j];
    let mut distances = vec![distance_row; max_i];

    distances[start_i][start_j] = 0;

    let mut queue = VecDeque::from([(start_i, start_j)]);
    while let Some((i, j)) = queue.pop_front() {
        let next_distance = distances[i][j] + 1;

        for (dx, dy) in dx.iter().copied().zip(dy.iter().copied()) {
            if let Some((next_i, next_j)) = offset_coords(i, j, dx, dy, max_i, max_j) {
                if heights[next_i][next_j] <= heights[i][j] + 1
                    && distances[next_i][next_j] == u64::MAX
                {
                    if (next_i, next_j) == (end_i, end_j) {
                        return next_distance;
                    }

                    distances[next_i][next_j] = next_distance;
                    queue.push_back((next_i, next_j));
                }
            }
        }
    }

    unreachable!("no solution found")
}

fn solve_part2(data: &[Vec<char>]) -> u64 {
    // Approach: start at E and go backwards to find the closest 'a' point.
    let (end_i, end_j) = locate_char(data, 'E').expect("no end");

    let heights = data
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match *c {
                    'S' => 0,
                    'E' => ('z' as u32) - ('a' as u32),
                    c => (c as u32) - ('a' as u32),
                })
                .collect_vec()
        })
        .collect_vec();

    let max_i = heights.len();
    let max_j = heights[0].len();

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    let distance_row = vec![u64::MAX; max_j];
    let mut distances = vec![distance_row; max_i];

    distances[end_i][end_j] = 0;

    let mut queue = VecDeque::from([(end_i, end_j)]);
    while let Some((i, j)) = queue.pop_front() {
        let next_distance = distances[i][j] + 1;

        for (dx, dy) in dx.iter().copied().zip(dy.iter().copied()) {
            if let Some((next_i, next_j)) = offset_coords(i, j, dx, dy, max_i, max_j) {
                if heights[i][j] <= heights[next_i][next_j] + 1
                    && distances[next_i][next_j] == u64::MAX
                {
                    if heights[next_i][next_j] == 0 {
                        return next_distance;
                    }

                    distances[next_i][next_j] = next_distance;
                    queue.push_back((next_i, next_j));
                }
            }
        }
    }

    unreachable!("no solution found")
}
