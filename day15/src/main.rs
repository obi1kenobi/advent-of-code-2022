use std::{collections::BTreeSet, env, fs, sync::Mutex};

use rayon::prelude::*;

#[allow(unused_imports)]
use itertools::Itertools;

use geometry::Vector2D;

mod geometry;

fn parse_point(point: &str) -> Vector2D {
    point
        .strip_prefix("x=")
        .expect("point doesn't start with 'x='")
        .split_once(", y=")
        .map(|(x, y)| {
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Vector2D::new(x, y)
        })
        .expect("point coordinates not delimited correctly")
}

fn parse_line(line: &str) -> (Vector2D, Vector2D) {
    line.strip_prefix("Sensor at ")
        .expect("bad line start")
        .trim_end()
        .split_once(": closest beacon is at ")
        .map(|(sensor, beacon)| (parse_point(sensor), parse_point(beacon)))
        .expect("split failed")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reversed_args: Vec<_> = args.iter().map(|x| x.as_str()).rev().collect();

    reversed_args
        .pop()
        .expect("Expected the executable name to be the first argument, but was missing");

    let part = reversed_args.pop().expect("part number");
    let input_file = reversed_args.pop().expect("input file");
    let content = fs::read_to_string(input_file).unwrap();

    let input_data: Vec<(Vector2D, Vector2D)> =
        content.trim_end().split('\n').map(parse_line).collect();

    match part {
        "1" => {
            #[allow(unused_variables)]
            let example_y: i64 = 10;

            #[allow(unused_variables)]
            let challenge_y: i64 = 2000000;

            let result = solve_part1(&input_data, challenge_y);
            println!("{result}");
        }
        "2" => {
            #[allow(unused_variables)]
            let example_cutoff: i64 = 20;

            #[allow(unused_variables)]
            let challenge_cutoff: i64 = 4000000;

            let result = solve_part2(&input_data, challenge_cutoff);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

fn cone_at_y(origin: Vector2D, y: i64, radius: i64) -> Option<(i64, i64)> {
    assert!(radius >= 0);
    let distance = (origin.y - y).abs();
    let leftover_distance = radius - distance;
    if leftover_distance < 0 {
        None
    } else {
        Some((origin.x - leftover_distance, origin.x + leftover_distance))
    }
}

fn solve_part1(data: &[(Vector2D, Vector2D)], target_y: i64) -> i64 {
    let mut endpoints = Vec::new();
    for (sensor, beacon) in data {
        let radius = (*sensor - *beacon).manhattan_length();
        if let Some((start_x, end_x)) = cone_at_y(*sensor, target_y, radius) {
            endpoints.push((start_x, Some(false)));
            endpoints.push((end_x, Some(true)));
        }

        // This beacon is relevant, add it to the sweep.
        // None sorts before any Some(x).
        if beacon.y == target_y {
            endpoints.push((beacon.x, None));
        }
    }

    let mut coverage_depth = 0;
    let mut denied_locations = 0;
    let mut coverage_began = None;

    endpoints.sort_unstable();

    // Sweep across the relevant coordinates.
    for (x_coord, is_end) in endpoints {
        if is_end.is_none() {
            // Found a beacon on this target y coordinate that's otherwise covered.
            if coverage_depth > 0 {
                denied_locations -= 1;
            }
        } else if let Some(is_end) = is_end {
            if coverage_depth == 0 {
                assert!(!is_end);
                coverage_began = Some(x_coord);
            }

            if !is_end {
                coverage_depth += 1;
            } else {
                coverage_depth -= 1;
            }

            if coverage_depth == 0 {
                let began = coverage_began.unwrap();
                denied_locations += x_coord + 1 - began;
            }
        }
    }

    assert!(denied_locations >= 0);
    denied_locations
}

fn allowed_location(
    data: &[(Vector2D, Vector2D)],
    beacons: &BTreeSet<Vector2D>,
    target_y: i64,
    cutoff_coord: i64,
) -> Option<Vector2D> {
    let mut endpoints = Vec::new();
    for (sensor, beacon) in data {
        let radius = (*sensor - *beacon).manhattan_length();
        if let Some((start_x, end_x)) = cone_at_y(*sensor, target_y, radius) {
            endpoints.push((start_x, false));
            endpoints.push((end_x, true));
        }
    }

    let mut coverage_depth = 0;
    let mut coverage_ended = Some(-1);

    endpoints.sort_unstable();

    // Sweep across the relevant coordinates.
    for (x_coord, is_end) in endpoints.iter().copied() {
        if x_coord > cutoff_coord && coverage_depth > 0 {
            break;
        }

        if let Some(ended_x) = coverage_ended {
            assert!(!is_end);

            for candidate_x in std::cmp::max(0, ended_x + 1)..x_coord {
                let candidate = Vector2D::new(candidate_x, target_y);
                if !beacons.contains(&candidate) {
                    return Some(candidate);
                }
            }
        }

        if !is_end {
            coverage_depth += 1;
        } else {
            coverage_depth -= 1;
        }

        if coverage_depth == 0 {
            coverage_ended = Some(x_coord);
        } else {
            coverage_ended = None;
        }
    }

    if let Some(ended_x) = coverage_ended {
        for candidate_x in std::cmp::max(0, ended_x + 1)..=cutoff_coord {
            let candidate = Vector2D::new(candidate_x, target_y);
            if !beacons.contains(&candidate) {
                return Some(candidate);
            }
        }
    }

    None
}

fn solve_part2(data: &[(Vector2D, Vector2D)], cutoff_coord: i64) -> i64 {
    let beacons = data.iter().map(|(_, beacon)| *beacon).collect();

    let mut answer: Mutex<Option<Vector2D>> = None.into();

    (0..=cutoff_coord).into_par_iter().for_each(|target_y| {
        if let Some(allowed) = allowed_location(data, &beacons, target_y, cutoff_coord) {
            let mut guard = answer.lock().unwrap();
            assert_eq!(*guard, None);
            *guard = Some(allowed);
        }
    });

    let allowed = answer
        .get_mut()
        .expect("failed to get mutex")
        .expect("no solution found");
    allowed.x * 4000000 + allowed.y
}
