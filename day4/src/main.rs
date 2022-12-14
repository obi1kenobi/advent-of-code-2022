use std::{env, fs, ops::RangeInclusive};

#[allow(unused_imports)]
use itertools::Itertools;

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let (from, to) = range.split_once('-').unwrap();
    from.parse().unwrap()..=to.parse().unwrap()
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

    let input_data: Vec<(RangeInclusive<i64>, RangeInclusive<i64>)> = content
        .trim_end()
        .split('\n')
        .map(|x| {
            let (left, right) = x.split_once(',').unwrap();
            (parse_range(left), parse_range(right))
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

fn fully_contains(bigger: &RangeInclusive<i64>, smaller: &RangeInclusive<i64>) -> bool {
    bigger.start() <= smaller.start() && bigger.end() >= smaller.end()
}

fn intersect(left: &RangeInclusive<i64>, right: &RangeInclusive<i64>) -> bool {
    let larger_start = std::cmp::max(left.start(), right.start());
    let smaller_end = std::cmp::min(left.end(), right.end());

    larger_start <= smaller_end
}

fn solve_part1(data: &[(RangeInclusive<i64>, RangeInclusive<i64>)]) -> usize {
    data.iter()
        .filter(|(left, right)| fully_contains(left, right) || fully_contains(right, left))
        .count()
}

fn solve_part2(data: &[(RangeInclusive<i64>, RangeInclusive<i64>)]) -> usize {
    data.iter()
        .filter(|(left, right)| intersect(left, right))
        .count()
}
