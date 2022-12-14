use std::{env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    // +1 = self win, -1 = self loss, 0 = tie
    fn outcome(&self, other: Shape) -> i64 {
        if self == &other {
            return 0;
        }

        match self {
            Shape::Rock => {
                if other == Shape::Paper {
                    -1
                } else {
                    1
                }
            }
            Shape::Paper => {
                if other == Shape::Scissors {
                    -1
                } else {
                    1
                }
            }
            Shape::Scissors => {
                if other == Shape::Rock {
                    -1
                } else {
                    1
                }
            }
        }
    }
}

fn score_round(opponent_play: Shape, my_play: Shape) -> u64 {
    let play_value = my_play.value();

    let outcome_value = match my_play.outcome(opponent_play) {
        -1 => 0,
        0 => 3,
        1 => 6,
        _ => unreachable!(),
    };

    play_value + outcome_value
}

fn suggest_play(opponent_play: Shape, outcome: i64) -> Shape {
    for shape in [Shape::Rock, Shape::Paper, Shape::Scissors] {
        if shape.outcome(opponent_play) == outcome {
            return shape;
        }
    }
    unreachable!()
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

    let input_data: Vec<(&str, &str)> = content
        .trim_end()
        .split('\n')
        .map(|x| x.trim().split_once(' ').unwrap())
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

fn solve_part1(data: &[(&str, &str)]) -> u64 {
    data.iter()
        .copied()
        .map(|(other, me)| {
            let opponent_play = match other {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let my_play = match me {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => unreachable!(),
            };

            score_round(opponent_play, my_play)
        })
        .sum()
}

fn solve_part2(data: &[(&str, &str)]) -> u64 {
    data.iter()
        .copied()
        .map(|(other, me)| {
            let opponent_play = match other {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let outcome = match me {
                "X" => -1,
                "Y" => 0,
                "Z" => 1,
                _ => unreachable!(),
            };
            let my_play = suggest_play(opponent_play, outcome);

            score_round(opponent_play, my_play)
        })
        .sum()
}
