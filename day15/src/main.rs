use std::{
    collections::BTreeSet,
    env, fs,
    ops::{Add, Neg, Sub},
};

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    const ALL_UNIT_VECTORS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn unit_vector(&self) -> Vector2D {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        let idx = unsafe { *<*const _>::from(self).cast::<u8>() } as usize;
        let (x, y) = Direction::ALL_UNIT_VECTORS[idx];
        Vector2D::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vector2D {
    x: i64,
    y: i64,
}

impl Vector2D {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        self + (-rhs)
    }
}

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
    line
        .strip_prefix("Sensor at ")
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

    let input_data: Vec<(Vector2D, Vector2D)> = content
        .trim_end()
        .split('\n')
        .map(parse_line)
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

fn solve_part1(data: &[(Vector2D, Vector2D)]) -> usize {
    todo!()
}

fn solve_part2(data: &[(Vector2D, Vector2D)]) -> usize {
    todo!()
}
