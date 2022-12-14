use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

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

    let input_data: &str = content.trim_end();

    match part {
        "1" => {
            let result = solve_part1(input_data);
            println!("{result}");
        }
        "2" => {
            let result = solve_part2(input_data);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

fn solve_part1(data: &str) -> usize {
    data.char_indices()
        .tuple_windows()
        .find_map(|((_, a), (_, b), (_, c), (idx, d))| {
            let chars = [a, b, c, d];
            (chars.into_iter().collect::<BTreeSet<_>>().len() == 4).then_some(idx + 1)
        })
        .expect("no solution found")
}

fn solve_part2(data: &str) -> usize {
    let length = 14;
    let char_indices = data.char_indices().collect_vec();

    let mut window: BTreeMap<char, usize> = Default::default();
    for (_, c) in char_indices.iter().take(length - 1) {
        *window.entry(*c).or_default() += 1;
    }

    let eviction_iter = char_indices.iter().map(|(_, c)| c);
    for ((idx, c), evicted_char) in char_indices
        .iter()
        .copied()
        .skip(length - 1)
        .zip(eviction_iter)
    {
        *window.entry(c).or_default() += 1;
        if window.len() == length {
            return idx + 1;
        }

        let removed_count = window.get_mut(evicted_char).unwrap();
        if *removed_count == 1 {
            window.remove(evicted_char);
        } else {
            *removed_count -= 1;
        }
    }

    unreachable!("no solution found")
}
