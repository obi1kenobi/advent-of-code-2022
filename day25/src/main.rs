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

    let input_data: Vec<&str> = content.trim_end().split('\n').collect();

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

fn from_snafu(value: &str) -> i64 {
    let mut result = 0;
    for c in value.chars() {
        result *= 5;
        result += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!("{c}"),
        };
    }
    result
}

fn to_snafu(mut value: i64) -> String {
    let mut result = vec![];

    while value != 0 {
        value += 2;
        let remainder = value % 5;
        let next_char = match remainder {
            0 => '=',
            1 => '-',
            r if r < 5 => char::from_digit((r - 2) as u32, 10).unwrap(),
            _ => unreachable!("{remainder}"),
        };
        result.push(next_char);
        value /= 5;
    }

    result.iter().rev().join("")
}

fn solve_part1(data: &[&str]) -> String {
    let total = data.iter().copied().map(from_snafu).sum();

    assert_eq!(total, from_snafu(to_snafu(total).as_str()));

    to_snafu(total)
}

fn solve_part2(_data: &[&str]) -> String {
    todo!()
}
