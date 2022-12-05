use std::{collections::VecDeque, env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let components = value.split(' ').collect_vec();
        assert_eq!(components[0], "move");
        assert_eq!(components[2], "from");
        assert_eq!(components[4], "to");
        Instruction {
            count: components[1].parse().unwrap(),
            from: components[3].parse::<usize>().unwrap() - 1,
            to: components[5].parse::<usize>().unwrap() - 1,
        }
    }
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

    let (stacks, instructions) = content
        .trim_end()
        .split_once("\n\n")
        .expect("no instruction separator found");
    let mut stack_lines: Vec<_> = stacks.split('\n').collect();
    let stack_ids = stack_lines.pop().unwrap();

    let stack_indices = stack_ids
        .char_indices()
        .filter_map(|(idx, c)| c.is_ascii_digit().then_some(idx))
        .collect_vec();

    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(stack_indices.len());
    stacks.resize(stack_indices.len(), Default::default());

    for line in stack_lines {
        let chars = line.chars().collect_vec();
        for (idx, offset) in stack_indices.iter().enumerate() {
            let c = chars[*offset];
            if c != ' ' {
                assert_ne!(c, '[');
                assert_ne!(c, ']');
                stacks.get_mut(idx).unwrap().push_back(c);
            }
        }
    }

    let instructions = instructions
        .split('\n')
        .map(Instruction::from)
        .collect_vec();

    match part {
        "1" => {
            let result = solve_part1(stacks, instructions);
            println!("{}", result);
        }
        "2" => {
            let result = solve_part2(stacks, instructions);
            println!("{}", result);
        }
        _ => unreachable!("{}", part),
    }
}

fn solve_part1(mut stacks: Vec<VecDeque<char>>, instructions: Vec<Instruction>) -> String {
    for instr in instructions {
        for _ in 0..instr.count {
            let container = stacks.get_mut(instr.from).unwrap().pop_front().unwrap();
            stacks.get_mut(instr.to).unwrap().push_front(container);
        }
    }

    stacks.iter().map(|s| s.front().unwrap()).collect()
}

fn solve_part2(mut stacks: Vec<VecDeque<char>>, instructions: Vec<Instruction>) -> String {
    for instr in instructions {
        let from = stacks.get_mut(instr.from).unwrap();
        let containers = (0..instr.count)
            .map(|_| from.pop_front().unwrap())
            .collect_vec();

        let to = stacks.get_mut(instr.to).unwrap();
        for container in containers.into_iter().rev() {
            to.push_front(container);
        }
    }

    stacks.iter().map(|s| s.front().unwrap()).collect()
}
