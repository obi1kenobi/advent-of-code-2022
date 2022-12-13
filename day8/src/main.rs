use std::{cmp::max, env, fs};

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

    let input_data: Vec<Vec<i64>> = content
        .trim_end()
        .split('\n')
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
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

fn calculate_directional_visibility_minimum(data: &[Vec<i64>], dx: i64, dy: i64) -> Vec<Vec<i64>> {
    let max_x = data.len();
    let max_y = data[0].len();

    let mut result = Vec::with_capacity(max_x);
    let empty_vec = vec![-1; max_y];
    result.resize(max_x, empty_vec);

    let x_indices = if dx > 0 {
        (0..max_x).collect_vec()
    } else {
        (0..max_x).rev().collect_vec()
    };
    let y_indices = if dy > 0 {
        (0..max_y).collect_vec()
    } else {
        (0..max_y).rev().collect_vec()
    };

    for x in x_indices.iter().copied() {
        for y in y_indices.iter().copied() {
            if let Some((new_x, new_y)) = offset_coords(x, y, dx, dy, max_x, max_y) {
                result[x][y] = max(result[new_x][new_y], data[new_x][new_y]);
            }
        }
    }

    result
}

fn solve_part1(data: &[Vec<i64>]) -> usize {
    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let visibilities = deltas
        .iter()
        .copied()
        .map(|(dx, dy)| calculate_directional_visibility_minimum(data, dx, dy))
        .collect_vec();

    data.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(y, value)| visibilities.iter().find(|vis| vis[x][y] < *value))
                .count()
        })
        .sum()
}

fn solve_part2(data: &[Vec<i64>]) -> usize {
    let max_x = data.len();
    let max_y = data[0].len();

    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    data.iter()
        .enumerate()
        .map(|(x, vals)| {
            vals.iter()
                .enumerate()
                .map(|(y, height)| {
                    let (scenic_1, scenic_2, scenic_3, scenic_4) = (0..4)
                        .map(|idx| {
                            let (dx, dy) = deltas[idx];

                            let mut count = 0;
                            let (mut cur_x, mut cur_y) = (x, y);
                            while let Some((next_x, next_y)) =
                                offset_coords(cur_x, cur_y, dx, dy, max_x, max_y)
                            {
                                count += 1;
                                if data[next_x][next_y] >= *height {
                                    break;
                                }

                                cur_x = next_x;
                                cur_y = next_y;
                            }

                            count
                        })
                        .collect_tuple()
                        .unwrap();

                    scenic_1 * scenic_2 * scenic_3 * scenic_4
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}
