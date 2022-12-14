use std::{env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

use nom::character::complete::{char, digit1, space0};
use nom::{IResult, Parser};
use nom_supreme::{error::ErrorTree, multi::collect_separated_terminated, parser_ext::ParserExt};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Number(i64),
    List(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::Number(a), Element::Number(b)) => a.cmp(b),
            (Element::Number(_), Element::List(_)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::List(_), Element::Number(_)) => self.cmp(&Element::List(vec![other.clone()])),
            (Element::List(a), Element::List(b)) => a
                .iter()
                .zip(b.iter())
                .find(|(l, r)| l != r)
                .map(|(l, r)| l.cmp(r))
                .unwrap_or_else(|| a.len().cmp(&b.len())),
        }
    }
}

fn number(input: &str) -> IResult<&str, i64, ErrorTree<&str>> {
    digit1.parse_from_str().parse(input)
}

fn nested_list(input: &str) -> IResult<&str, Element, ErrorTree<&str>> {
    let mut parser = collect_separated_terminated(
        number
            .terminated(space0)
            .map(Element::Number)
            .or(nested_list),
        char(',').terminated(space0),
        char(']'),
    )
    .or(char(']').value(vec![]))
    .preceded_by(char('[').terminated(space0));

    parser
        .parse(input)
        .map(|(input, res)| (input, Element::List(res)))
}

impl Element {
    fn parse(input: &str) -> Self {
        let (input, parsed) = nested_list(input).expect("parse failed");
        assert!(input.is_empty());
        parsed
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

    let input_data: Vec<(Element, Element)> = content
        .trim_end()
        .split("\n\n")
        .map(|x| {
            let (first, second) = x.split_once('\n').unwrap();
            (Element::parse(first), Element::parse(second))
        })
        .collect();

    match part {
        "1" => {
            let result = solve_part1(&input_data);
            println!("{result}");
        }
        "2" => {
            let result = solve_part2(input_data);
            println!("{result}");
        }
        _ => unreachable!("{}", part),
    }
}

fn solve_part1(data: &[(Element, Element)]) -> usize {
    data.iter()
        .enumerate()
        .filter_map(|(idx, (left, right))| (left <= right).then_some(idx + 1))
        .sum()
}

fn solve_part2(data: Vec<(Element, Element)>) -> usize {
    let divider_a = Element::List(vec![Element::List(vec![Element::Number(2)])]);
    let divider_b = Element::List(vec![Element::List(vec![Element::Number(6)])]);

    let mut all_packets = Vec::with_capacity(data.len() + 2);
    all_packets.extend(data.into_iter().flat_map(|(a, b)| [a, b].into_iter()));
    all_packets.push(divider_a.clone());
    all_packets.push(divider_b.clone());

    all_packets.sort_unstable();
    let (idx_a, _) = all_packets
        .iter()
        .find_position(|p| p == &&divider_a)
        .expect("divider_a not found");
    let (idx_b, _) = all_packets
        .iter()
        .find_position(|p| p == &&divider_b)
        .expect("divider_b not found");

    (idx_a + 1) * (idx_b + 1)
}
