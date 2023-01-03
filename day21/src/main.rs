use std::{collections::BTreeMap, env, fs};

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation<'a> {
    Literal(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
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

    let input_data: BTreeMap<&str, Operation<'_>> = content
        .trim_end()
        .split('\n')
        .map(|x| {
            let (name, rest) = x.split_once(": ").unwrap();
            let operation = if let Ok(num) = rest.parse() {
                Operation::Literal(num)
            } else if let Some((left, right)) = rest.split_once(" + ") {
                Operation::Add(left, right)
            } else if let Some((left, right)) = rest.split_once(" - ") {
                Operation::Sub(left, right)
            } else if let Some((left, right)) = rest.split_once(" * ") {
                Operation::Mul(left, right)
            } else if let Some((left, right)) = rest.split_once(" / ") {
                Operation::Div(left, right)
            } else {
                unreachable!("unexpected operation {rest}")
            };
            (name, operation)
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

fn evaluate<'a>(
    name: &'a str,
    values: &mut BTreeMap<&'a str, i64>,
    operations: &BTreeMap<&'a str, Operation<'a>>,
) -> i64 {
    if let Some(num) = values.get(name) {
        return *num;
    }

    let value = match operations[&name] {
        Operation::Literal(l) => l,
        Operation::Add(l, r) => evaluate(l, values, operations) + evaluate(r, values, operations),
        Operation::Sub(l, r) => evaluate(l, values, operations) - evaluate(r, values, operations),
        Operation::Mul(l, r) => evaluate(l, values, operations) * evaluate(r, values, operations),
        Operation::Div(l, r) => evaluate(l, values, operations) / evaluate(r, values, operations),
    };
    values.insert(name, value);

    value
}

fn solve_part1(operations: &BTreeMap<&str, Operation<'_>>) -> i64 {
    let mut values: BTreeMap<&str, i64> = operations
        .iter()
        .filter_map(|(k, v)| {
            if let Operation::Literal(num) = v {
                Some((*k, *num))
            } else {
                None
            }
        })
        .collect();

    evaluate("root", &mut values, operations)
}

fn evaluate_except_special<'a>(
    name: &'a str,
    values: &mut BTreeMap<&'a str, i64>,
    operations: &BTreeMap<&'a str, Operation<'a>>,
) -> Option<i64> {
    if let Some(num) = values.get(name) {
        return Some(*num);
    }

    let value = match operations[&name] {
        Operation::Literal(l) => l,
        Operation::Add(l, r) => {
            evaluate_except_special(l, values, operations)?
                + evaluate_except_special(r, values, operations)?
        }
        Operation::Sub(l, r) => {
            evaluate_except_special(l, values, operations)?
                - evaluate_except_special(r, values, operations)?
        }
        Operation::Mul(l, r) => {
            evaluate_except_special(l, values, operations)?
                * evaluate_except_special(r, values, operations)?
        }
        Operation::Div(l, r) => {
            evaluate_except_special(l, values, operations)?
                / evaluate_except_special(r, values, operations)?
        }
    };

    // Return late so that we evaluate as many expressions as possible.
    if name == "humn" || name == "root" {
        return None;
    }

    values.insert(name, value);

    Some(value)
}

fn evaluate_backward<'a>(
    name: &'a str,
    expected_result: i64,
    values: &mut BTreeMap<&'a str, i64>,
    operations: &BTreeMap<&'a str, Operation<'a>>,
) -> Result<(), ()> {
    match operations[&name] {
        Operation::Literal(num) => {
            if name != "humn" {
                assert_eq!(expected_result, num);
                Ok(())
            } else {
                Ok(())
            }
        }
        op => {
            let (l, r) = match op {
                Operation::Literal(_) => unreachable!(),
                Operation::Add(l, r)
                | Operation::Sub(l, r)
                | Operation::Mul(l, r)
                | Operation::Div(l, r) => (l, r),
            };
            let maybe_left = values.get(l).copied();
            let maybe_right = values.get(r).copied();
            match (maybe_left, maybe_right) {
                (Some(num), None) => {
                    let other = match op {
                        Operation::Literal(_) => unreachable!(),
                        Operation::Add(_, _) => expected_result - num,
                        Operation::Sub(_, _) => num - expected_result,
                        Operation::Mul(_, _) => {
                            if num == 0 {
                                return Err(());
                            } else {
                                let value = expected_result / num;
                                if value * num != expected_result {
                                    return Err(());
                                }
                                value
                            }
                        }
                        Operation::Div(_, _) => {
                            if expected_result == 0 {
                                return Err(());
                            } else {
                                num / expected_result
                            }
                        }
                    };
                    values.insert(r, other);
                    let result = evaluate_backward(r, other, values, operations);
                    if result.is_err() {
                        values.remove(r);
                    }
                    result
                }
                (None, Some(num)) => {
                    let other = match op {
                        Operation::Literal(_) => unreachable!(),
                        Operation::Add(_, _) => expected_result - num,
                        Operation::Sub(_, _) => expected_result + num,
                        Operation::Mul(_, _) => {
                            if expected_result == 0 {
                                return Err(());
                            } else {
                                let value = expected_result / num;
                                if num * value != expected_result {
                                    return Err(());
                                }
                                value
                            }
                        }
                        Operation::Div(_, _) => expected_result * num,
                    };
                    values.insert(l, other);
                    let result = evaluate_backward(l, other, values, operations);
                    if result.is_err() {
                        values.remove(l);
                    }
                    result
                }
                _ => Ok(()),
            }
        }
    }
}

fn solve_part2(operations: &BTreeMap<&str, Operation<'_>>) -> i64 {
    let mut values: BTreeMap<&str, i64> = operations
        .iter()
        .filter_map(|(k, v)| {
            if let Operation::Literal(num) = v {
                (*k != "root" && *k != "humn").then_some((*k, *num))
            } else {
                None
            }
        })
        .collect();

    let (left, right) = match operations["root"] {
        Operation::Literal(_) => unreachable!(),
        Operation::Add(left, right)
        | Operation::Sub(left, right)
        | Operation::Mul(left, right)
        | Operation::Div(left, right) => (left, right),
    };

    let left_result = evaluate_except_special(left, &mut values, operations);
    let right_result = evaluate_except_special(right, &mut values, operations);

    match (left_result, right_result) {
        (Some(left), None) => {
            evaluate_backward(right, left, &mut values, operations).expect("right failed");
        }
        (None, Some(right)) => {
            evaluate_backward(left, right, &mut values, operations).expect("left failed");
        }
        _ => unreachable!("{left_result:?} {right_result:?}"),
    }

    while !values.contains_key("humn") {
        for name in operations.keys().copied() {
            if name != "root" && name != "humn" {
                evaluate_except_special(name, &mut values, operations);
            }
        }

        for name in operations.keys().copied() {
            let Some(op_value) = values.get(name).copied() else { continue };
            evaluate_backward(name, op_value, &mut values, operations)
                .expect("backward computation failed");
        }
    }

    values["humn"]
}
