use std::{collections::BTreeSet, env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

fn priority(c: char) -> i64 {
    let value = if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    };
    value as i64
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

    let input_data: Vec<&str> = content.trim_end().split('\n').collect();

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

fn solve_part1(data: &[&str]) -> i64 {
    data.iter()
        .map(|rucksack| {
            let (front, back) = rucksack.split_at(rucksack.len() / 2);
            let front_chars: BTreeSet<_> = front.chars().collect();

            for back_char in back.chars() {
                if front_chars.contains(&back_char) {
                    return priority(back_char);
                }
            }
            unreachable!()
        })
        .sum()
}

fn solve_part2(data: &[&str]) -> i64 {
    data.iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let (first, second, third) = chunk.copied().collect_tuple().unwrap();

            let first_chars = first.chars().collect::<BTreeSet<_>>();
            let second_chars = second.chars().collect::<BTreeSet<_>>();
            let third_chars = third.chars().collect::<BTreeSet<_>>();

            let first_intersect = first_chars
                .intersection(&second_chars)
                .copied()
                .collect::<BTreeSet<_>>();

            let common_item = first_intersect.intersection(&third_chars).next().unwrap();

            priority(*common_item)
        })
        .sum()
}
