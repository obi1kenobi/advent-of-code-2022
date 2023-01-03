use std::{collections::BTreeSet, env, fs};

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

    let input_data: Vec<(i64, i64, i64)> = content
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
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

fn solve_part1(data: &[(i64, i64, i64)]) -> usize {
    let cubes: BTreeSet<_> = data.iter().copied().collect();
    let mut faces = data.len() * 6;

    for cube in data {
        for i in [-1, 1] {
            let (x, y, z) = *cube;
            if cubes.contains(&(x + i, y, z)) {
                faces -= 1;
            }
            if cubes.contains(&(x, y + i, z)) {
                faces -= 1;
            }
            if cubes.contains(&(x, y, z + i)) {
                faces -= 1;
            }
        }
    }

    faces
}

type Point3D = (i64, i64, i64);

fn calculate_bounding_box(cubes: &[Point3D]) -> (Point3D, Point3D) {
    let minx = cubes.iter().copied().map(|(x, _, _)| x).min().unwrap();
    let miny = cubes.iter().copied().map(|(_, y, _)| y).min().unwrap();
    let minz = cubes.iter().copied().map(|(_, _, z)| z).min().unwrap();

    let maxx = cubes.iter().copied().map(|(x, _, _)| x).max().unwrap();
    let maxy = cubes.iter().copied().map(|(_, y, _)| y).max().unwrap();
    let maxz = cubes.iter().copied().map(|(_, _, z)| z).max().unwrap();

    ((minx, miny, minz), (maxx, maxy, maxz))
}

fn inside_bounding_box(bounding_box: (Point3D, Point3D), cube: Point3D) -> bool {
    let (x, y, z) = cube;
    let ((minx, miny, minz), (maxx, maxy, maxz)) = bounding_box;

    x >= minx && x <= maxx && y >= miny && y <= maxy && z >= minz && z <= maxz
}

fn count_visible(
    cubes: &BTreeSet<Point3D>,
    bounding_box: (Point3D, Point3D),
    visited: &mut BTreeSet<Point3D>,
    current: Point3D,
) -> usize {
    if !visited.insert(current) {
        // Already visited previously.
        return 0;
    }

    let mut visible = 0;
    for i in [-1, 1] {
        let (x, y, z) = current;

        for next_cube in [(x + i, y, z), (x, y + i, z), (x, y, z + i)] {
            if inside_bounding_box(bounding_box, next_cube) {
                if cubes.contains(&next_cube) {
                    visible += 1;
                } else {
                    visible += count_visible(cubes, bounding_box, visited, next_cube);
                }
            }
        }
    }

    visible
}

fn solve_part2(data: &[Point3D]) -> usize {
    let bounding_box = calculate_bounding_box(data);
    let ((minx, miny, minz), (maxx, maxy, maxz)) = bounding_box;

    let cubes: BTreeSet<_> = data.iter().copied().collect();
    let mut visible = 0;
    let mut visited: BTreeSet<_> = Default::default();

    for y in miny..=maxy {
        for z in minz..=maxz {
            visible += count_visible(&cubes, bounding_box, &mut visited, (minx - 1, y, z));
            visible += count_visible(&cubes, bounding_box, &mut visited, (maxx + 1, y, z));
        }
    }
    for x in minx..=maxx {
        for y in miny..=maxy {
            visible += count_visible(&cubes, bounding_box, &mut visited, (x, y, minz - 1));
            visible += count_visible(&cubes, bounding_box, &mut visited, (x, y, maxz + 1));
        }
    }
    for x in minx..=maxx {
        for z in minz..=maxz {
            visible += count_visible(&cubes, bounding_box, &mut visited, (x, miny - 1, z));
            visible += count_visible(&cubes, bounding_box, &mut visited, (x, maxy + 1, z));
        }
    }

    visible
}
