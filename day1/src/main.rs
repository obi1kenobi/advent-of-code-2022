use std::{env, fs};

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

    let input_data: Vec<Vec<u64>> = content
        .trim_end()
        .split("\n\n")
        .map(|x| {
            x.trim_end()
                .split('\n')
                .map(|x| x.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    match part {
        "1" => {
            let result = solve_part1(&input_data);
            println!("{}", result);
        }
        "2" => {
            let result = solve_part2(&input_data);
            println!("{}", result);
        }
        _ => unreachable!("{}", part),
    }
}

fn solve_part1(data: &[Vec<u64>]) -> u64 {
    data.iter().map(|x| x.iter().sum()).max().unwrap()
}

fn solve_part2(data: &[Vec<u64>]) -> u64 {
    data.iter()
        .map(|x| x.iter().sum::<u64>())
        .sorted_unstable()
        .rev()
        .take(3)
        .sum()
}
