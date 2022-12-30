use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

use geometry::Vector2D;
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

    let input_data: Vec<Vec<(i64, i64)>> = content
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(" -> ")
                .map(|tuple| {
                    tuple
                        .split_once(',')
                        // flip the coordinates so the axes are (down, right)
                        .map(|(a, b)| (b.parse().unwrap(), a.parse().unwrap()))
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let lines: BTreeSet<_> = input_data
        .iter()
        .flat_map(|segment| {
            segment
                .iter()
                .tuple_windows()
                .map(|(pt_a, pt_b)| Line::new(*pt_a, *pt_b))
        })
        .collect();

    let mut lowest_point_per_vertical = BTreeMap::new();
    for line in &lines {
        let start_pt = line.start.point;
        let end_pt = line.end.point;

        if line.is_horizontal {
            assert_eq!(start_pt.x, end_pt.x);
            assert!(start_pt.y <= end_pt.y);
            for y in start_pt.y..=end_pt.y {
                lowest_point_per_vertical
                    .entry(y)
                    .and_modify(|value| *value = max(*value, start_pt.x))
                    .or_insert(start_pt.x);
            }
        } else {
            assert_eq!(start_pt.y, end_pt.y);
            assert!(start_pt.x <= end_pt.x);

            lowest_point_per_vertical
                .entry(start_pt.y)
                .and_modify(|value| *value = max(*value, end_pt.x))
                .or_insert(end_pt.x);
        }
    }

    match part {
        "1" => {
            let result = solve_part1(lines, lowest_point_per_vertical);
            println!("{result}");
        }
        "2" => {
            let result = solve_part2(lines, lowest_point_per_vertical);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LineEndpoint {
    point: Vector2D,
    line_end: bool,
}

impl LineEndpoint {
    fn new(x: i64, y: i64, line_end: bool) -> Self {
        Self {
            point: Vector2D::new(x, y),
            line_end,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    start: LineEndpoint,
    end: LineEndpoint,
    is_horizontal: bool,
}

impl Line {
    fn new(pt_a: (i64, i64), pt_b: (i64, i64)) -> Self {
        let (a_x, a_y) = pt_a;
        let (b_x, b_y) = pt_b;

        let is_horizontal = a_x == b_x;

        let (start, end) = if a_x == b_x {
            if a_y <= b_y {
                let endpoint_a = LineEndpoint::new(a_x, a_y, false);
                let endpoint_b = LineEndpoint::new(b_x, b_y, true);
                (endpoint_a, endpoint_b)
            } else {
                let endpoint_a = LineEndpoint::new(a_x, a_y, true);
                let endpoint_b = LineEndpoint::new(b_x, b_y, false);
                (endpoint_b, endpoint_a)
            }
        } else if a_y == b_y {
            if a_x <= b_x {
                let endpoint_a = LineEndpoint::new(a_x, a_y, false);
                let endpoint_b = LineEndpoint::new(b_x, b_y, true);
                (endpoint_a, endpoint_b)
            } else {
                let endpoint_a = LineEndpoint::new(a_x, a_y, true);
                let endpoint_b = LineEndpoint::new(b_x, b_y, false);
                (endpoint_b, endpoint_a)
            }
        } else {
            unreachable!("line is neither horizontal nor vertical: {pt_a:?}, {pt_b:?}")
        };

        assert!(!start.line_end);
        assert!(end.line_end);
        Self {
            start,
            end,
            is_horizontal,
        }
    }
}

const SAND_ORIGIN: Vector2D = Vector2D::new(0, 500);

struct LocalMap {
    map: Vec<[bool; 1024]>,
    origin: Vector2D,
}

impl LocalMap {
    fn new(origin: Vector2D) -> Self {
        Self {
            map: vec![[false; 1024]; 1024],
            origin,
        }
    }

    fn new_with_centered_point(pt: Vector2D) -> Self {
        let target_pt = Vector2D::new(0, 511);
        Self::new(target_pt - pt)
    }

    fn get_pt(&self, pt: Vector2D) -> bool {
        let new_pt = pt + self.origin;
        self.map[new_pt.x as usize][new_pt.y as usize]
    }

    #[allow(dead_code)]
    fn get(&self, x: i64, y: i64) -> bool {
        self.get_pt(Vector2D::new(x, y))
    }

    fn get_pt_mut(&mut self, pt: Vector2D) -> &mut bool {
        let new_pt = pt + self.origin;
        self.map
            .get_mut(new_pt.x as usize)
            .unwrap()
            .get_mut(new_pt.y as usize)
            .unwrap()
    }

    fn get_mut(&mut self, x: i64, y: i64) -> &mut bool {
        self.get_pt_mut(Vector2D::new(x, y))
    }

    fn near_boundary(&self, pt: Vector2D) -> bool {
        let new_pt = pt + self.origin;

        // ignore being near the top boundary, since sand only falls down
        new_pt.y < 2 || new_pt.x >= 1020 || new_pt.y >= 1020
    }
}

const DROP_DIRECTIONS: [Vector2D; 3] = [
    Vector2D::new(1, 0),  // down
    Vector2D::new(1, -1), // down + left
    Vector2D::new(1, 1),  // down + right
];

fn populate_map(
    local_map: &mut LocalMap,
    sand_points: &BTreeSet<Vector2D>,
    lines: &BTreeSet<Line>,
) {
    for sand_pt in sand_points.iter() {
        *local_map.get_pt_mut(*sand_pt) = true;
    }

    for line in lines {
        let start_pt = line.start.point;
        let end_pt = line.end.point;

        if line.is_horizontal {
            assert_eq!(start_pt.x, end_pt.x);
            assert!(start_pt.y <= end_pt.y);
            for y in start_pt.y..=end_pt.y {
                *local_map.get_mut(start_pt.x, y) = true;
            }
        } else {
            assert_eq!(start_pt.y, end_pt.y);
            assert!(start_pt.x <= end_pt.x);
            for x in start_pt.x..=end_pt.x {
                *local_map.get_mut(x, start_pt.y) = true;
            }
        }
    }
}

fn drop_sand(
    local_map: &mut LocalMap,
    sand_points: &mut BTreeSet<Vector2D>,
    lines: &BTreeSet<Line>,
    lowest_point_per_vertical: &BTreeMap<i64, i64>,
    sand_source: Vector2D,
) -> Option<Vector2D> {
    if local_map.near_boundary(sand_source) {
        let mut new_local_map = LocalMap::new_with_centered_point(sand_source);
        let local_map = &mut new_local_map;
        populate_map(local_map, sand_points, lines);
        drop_sand(
            local_map,
            sand_points,
            lines,
            lowest_point_per_vertical,
            sand_source,
        )
    } else {
        for offset in DROP_DIRECTIONS.iter() {
            let next_point = sand_source + *offset;
            if !local_map.get_pt(next_point) {
                // Check if the sand is about to fall into infinity.
                if let Some(lowest_coord) = lowest_point_per_vertical.get(&next_point.y) {
                    if *lowest_coord < next_point.x {
                        return Some(sand_source);
                    }
                } else {
                    // There are no walls in this vertical column at all.
                    // The sand is going to fall into infinity.
                    return Some(sand_source);
                }

                // Drop the sand to the next point. If it ends up falling into infinity,
                // stop dropping sand.
                let final_sand_source = drop_sand(
                    local_map,
                    sand_points,
                    lines,
                    lowest_point_per_vertical,
                    next_point,
                );
                if final_sand_source.is_some() {
                    return final_sand_source;
                }
            }
        }

        sand_points.insert(sand_source);
        *local_map.get_pt_mut(sand_source) = true;
        None
    }
}

fn solve_part1(lines: BTreeSet<Line>, lowest_point_per_vertical: BTreeMap<i64, i64>) -> usize {
    let mut sand_points: BTreeSet<Vector2D> = Default::default();

    let mut local_map = LocalMap::new(Vector2D::new(0, 0));
    populate_map(&mut local_map, &sand_points, &lines);
    let stopping = drop_sand(
        &mut local_map,
        &mut sand_points,
        &lines,
        &lowest_point_per_vertical,
        SAND_ORIGIN,
    );
    assert!(stopping.is_some());

    sand_points.len()
}

fn solve_part2(
    mut lines: BTreeSet<Line>,
    mut lowest_point_per_vertical: BTreeMap<i64, i64>,
) -> usize {
    let lowest_horizontal = *lowest_point_per_vertical.values().max().unwrap();
    let floor_x = lowest_horizontal + 2;

    let floor_y_min = SAND_ORIGIN.y - floor_x - 1;
    let floor_y_max = SAND_ORIGIN.y + floor_x + 1;
    lines.insert(Line::new((floor_x, floor_y_min), (floor_x, floor_y_max)));
    for y in floor_y_min..=floor_y_max {
        lowest_point_per_vertical.insert(y, floor_x);
    }

    let mut sand_points: BTreeSet<Vector2D> = Default::default();

    let mut local_map = LocalMap::new(Vector2D::new(0, 0));
    populate_map(&mut local_map, &sand_points, &lines);
    let stopping = drop_sand(
        &mut local_map,
        &mut sand_points,
        &lines,
        &lowest_point_per_vertical,
        SAND_ORIGIN,
    );
    assert!(stopping.is_none());
    assert!(local_map.get_pt(SAND_ORIGIN));

    sand_points.len()
}
