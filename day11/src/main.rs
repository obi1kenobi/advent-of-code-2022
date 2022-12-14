use std::{
    collections::{BTreeMap, VecDeque},
    env, fs,
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use nom::Parser;

#[allow(unused_imports)]
use nom_supreme::parser_ext::ParserExt;

use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, multispace1, space0, space1},
    multi::many1,
    sequence::tuple,
    IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

fn parse_number(input: &str) -> IResult<&str, i64, ErrorTree<&str>> {
    digit1.parse_from_str().parse(input)
}

#[derive(Debug, Clone)]
enum Operand {
    Input,
    Literal(i64),
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
        alt((
            tag("old").map(|_| Operand::Input),
            parse_number.map(Operand::Literal),
        ))(input)
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Operand),
    Mul(Operand),
}

impl Operation {
    fn apply(&self, input: i64) -> i64 {
        match self {
            Operation::Add(operand) => {
                input
                    + match operand {
                        Operand::Input => input,
                        Operand::Literal(x) => *x,
                    }
            }
            Operation::Mul(operand) => {
                input
                    * match operand {
                        Operand::Input => input,
                        Operand::Literal(x) => *x,
                    }
            }
        }
    }

    fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
        tag("new = old ")
            .precedes(alt((
                char('+')
                    .precedes(space1)
                    .precedes(Operand::parse)
                    .map(Operation::Add),
                char('*')
                    .precedes(space1)
                    .precedes(Operand::parse)
                    .map(Operation::Mul),
            )))
            .parse(input)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    queue: VecDeque<i64>,
    operation: Operation,
    test_divisor: i64,
    target_when_true: i64,
    target_when_false: i64,
}

#[derive(Debug, Clone)]
struct MonkeyBusiness {
    monkeys: BTreeMap<i64, Monkey>,
    items_inspected: BTreeMap<i64, usize>,
}

impl MonkeyBusiness {
    fn new(monkeys: BTreeMap<i64, Monkey>) -> Self {
        Self {
            items_inspected: monkeys.keys().map(|k| (*k, 0)).collect(),
            monkeys,
        }
    }

    fn parse(mut input: &str) -> Self {
        let mut monkey_parser = tuple((
            tag("Monkey")
                .terminated(space1)
                .precedes(parse_number.terminated(char(':')))
                .terminated(multispace1),
            tag("Starting items: ").precedes(
                many1(parse_number.terminated(char(',').terminated(space0).opt()))
                    .terminated(multispace1),
            ),
            tag("Operation: ")
                .precedes(Operation::parse)
                .terminated(multispace1),
            tag("Test: divisible by ")
                .precedes(parse_number)
                .terminated(multispace1),
            tag("If true: throw to monkey ")
                .precedes(parse_number)
                .terminated(multispace1),
            tag("If false: throw to monkey ")
                .precedes(parse_number)
                .terminated(multispace0),
        ));

        let mut monkeys = BTreeMap::new();

        while !input.is_empty() {
            let (
                next_input,
                (id, starting_items, operation, test_divisor, target_when_true, target_when_false),
            ) = monkey_parser.parse(input).expect("parse failed");
            input = next_input;

            let monkey = Monkey {
                queue: starting_items.into(),
                operation,
                test_divisor,
                target_when_true,
                target_when_false,
            };
            monkeys.insert(id, monkey);
        }

        Self::new(monkeys)
    }

    fn simulate_monkey(&mut self, id: i64, worry_reduction: &impl Fn(i64) -> i64) {
        while let Some(item_value) = self.monkeys.get_mut(&id).unwrap().queue.pop_front() {
            let monkey = &self.monkeys[&id];
            let new_worry = worry_reduction(monkey.operation.apply(item_value));

            let next_id = if new_worry % monkey.test_divisor == 0 {
                monkey.target_when_true
            } else {
                monkey.target_when_false
            };
            self.monkeys
                .get_mut(&next_id)
                .unwrap()
                .queue
                .push_back(new_worry);

            self.items_inspected.entry(id).and_modify(|x| *x += 1);
        }
    }

    fn simulate_round(&mut self, worry_reduction: impl Fn(i64) -> i64) {
        for index in 0..self.monkeys.len() {
            self.simulate_monkey(index as i64, &worry_reduction);
        }
    }

    fn get_monkey_business(&self) -> usize {
        let most_items = *self.items_inspected.values().max().unwrap();
        let (_, &mut second_most_items, _) = self
            .items_inspected
            .values()
            .collect_vec()
            .select_nth_unstable(self.items_inspected.len() - 2);

        most_items * second_most_items
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

    let input_data: MonkeyBusiness = MonkeyBusiness::parse(&content);

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

fn solve_part1(mut data: MonkeyBusiness) -> usize {
    for _ in 0..20 {
        data.simulate_round(|worry| worry / 3);
    }

    data.get_monkey_business()
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        1 => 1,
        b if a < b => gcd(b, a),
        b => gcd(b, a % b),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    let common = gcd(a, b);
    a / common * b
}

fn solve_part2(mut data: MonkeyBusiness) -> usize {
    let divisors_lcm = data.monkeys.values().map(|m| m.test_divisor).fold(1, lcm);

    for _ in 0..10000 {
        data.simulate_round(|worry| worry % divisors_lcm);
    }

    data.get_monkey_business()
}
