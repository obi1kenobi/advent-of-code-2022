use std::{env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use nom::Parser;

#[allow(unused_imports)]
use nom_supreme::parser_ext::ParserExt;

use nom::{branch::alt, character::complete::space1};
use nom_supreme::tag::complete::tag;

#[derive(Debug, Clone)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let mut parser = alt((
            tag("noop").value(Instruction::Noop),
            tag("addx")
                .precedes(space1)
                .precedes(nom::character::complete::i64::<&str, nom::error::Error<&str>>)
                .map(Instruction::Addx),
        ))
        .complete();

        let (remainder, result) = parser.parse(input).unwrap();
        assert!(remainder.is_empty());
        result
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

    let input_data: Vec<Instruction> = content
        .trim_end()
        .split('\n')
        .map(Instruction::parse)
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

fn solve_part1(data: &[Instruction]) -> i64 {
    let mut x_register = 1i64;

    let signal_strength_reading_stride = 40;
    let mut cycles_remaining = 20;
    let mut cycle_counter = 0;
    let mut result = 0;

    for instr in data {
        let (addend, latency) = match instr {
            Instruction::Addx(n) => (*n, 2),
            Instruction::Noop => (0, 1),
        };

        assert!(cycles_remaining >= 0);
        if cycles_remaining <= latency {
            result += (cycle_counter + cycles_remaining) * x_register;
            cycles_remaining += signal_strength_reading_stride;
        }

        cycles_remaining -= latency;
        cycle_counter += latency;
        x_register += addend;
    }

    result
}

fn solve_part2(data: &[Instruction]) -> String {
    let mut x_register = 1i64;
    let mut result = String::with_capacity(250);

    let screen_width = 40;
    let mut cycle_counter = 0;

    for instr in data {
        let (addend, latency) = match instr {
            Instruction::Addx(n) => (*n, 2),
            Instruction::Noop => (0, 1),
        };

        for _ in 0..latency {
            let next_char = if ((x_register - 1)..=(x_register + 1)).contains(&cycle_counter) {
                "#"
            } else {
                "."
            };
            cycle_counter += 1;
            result += next_char;

            if cycle_counter == screen_width {
                result += "\n";
                cycle_counter -= screen_width
            }
        }

        x_register += addend;
    }

    result
}
