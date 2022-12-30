use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

#[allow(unused_imports)]
use itertools::Itertools;

struct Valve {
    name: String,
    flow: i64,
    neighbors: Vec<String>,
}

impl Valve {
    fn parse(line: &str) -> Self {
        let line = line.strip_prefix("Valve ").unwrap();
        let (name, rest) = line.split_once(" has flow rate=").unwrap();
        let (flow, neighbors) = rest
            .split_once("; tunnels lead to valves ")
            .unwrap_or_else(|| rest.split_once("; tunnel leads to valve ").unwrap());

        let neighbors = neighbors
            .trim_end()
            .split(", ")
            .map(|x| x.to_string())
            .collect();
        Self {
            name: name.to_string(),
            flow: flow.parse().unwrap(),
            neighbors,
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

    let input_data: Vec<Valve> = content.trim_end().lines().map(Valve::parse).collect();

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

fn fill_dp_matrix(
    dp: &mut [Vec<Vec<i64>>],
    data: &[Valve],
    non_zero_valve_names: &[&str],
    max_time: usize,
) {
    let all_names: BTreeMap<_, _> = data
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.as_str(), i))
        .collect();
    let non_zero_indexes: BTreeMap<_, _> = non_zero_valve_names
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect();

    // Run Floyd-Warshall to compute the all-pairs shortest paths.
    let inf_dist = 10000;
    let mut dist = vec![vec![inf_dist; data.len()]; data.len()];
    for (i, valve) in data.iter().enumerate() {
        for neighbor in valve.neighbors.iter() {
            let j = all_names[neighbor.as_str()];
            dist[i][j] = 1;
        }
    }
    for k in 0..data.len() {
        for i in 0..data.len() {
            for j in 0..data.len() {
                dist[i][j] = std::cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    // Run dynamic programming over the 2^(non-zero valve) * current location * time domain.
    assert!(non_zero_valve_names.len() < 20);
    let mut next_states: BTreeSet<(usize, usize, usize)> = Default::default();

    // Ensure valve AA has zero flow rate (it is defined as being jammed).
    // Given that, it isn't one of our non-zero valves in the DP.
    let aa_valve_index = all_names["AA"];
    assert_eq!(data[aa_valve_index].flow, 0);

    // Travel to all the non-zero neighbors of AA and open the valves there to begin.
    for non_zero_valve in non_zero_valve_names {
        let index = all_names[*non_zero_valve];
        let time_to_reach_from_aa = dist[aa_valve_index][index];
        assert!(time_to_reach_from_aa < inf_dist);

        let next_time = time_to_reach_from_aa + 1;
        let nonzero_index = non_zero_indexes[non_zero_valve];
        let opened_valves = 1 << nonzero_index;
        if next_time <= max_time {
            dp[next_time][nonzero_index][opened_valves] =
                data[index].flow * (max_time as i64 - next_time as i64);
            next_states.insert((next_time, nonzero_index, opened_valves));
        }
    }

    while let Some((time, location_index, opened_valves)) = next_states.pop_first() {
        #[allow(clippy::needless_range_loop)]
        for nonzero_index in 0..non_zero_valve_names.len() {
            let valve_flag = 1 << nonzero_index;
            let next_opened_valves = opened_valves | valve_flag;
            if next_opened_valves == opened_valves {
                // The valve was already open.
                continue;
            }

            let location_valve_index = all_names[non_zero_valve_names[location_index]];
            let valve_index = all_names[non_zero_valve_names[nonzero_index]];
            let next_time = time + dist[location_valve_index][valve_index] + 1;
            if next_time <= max_time {
                dp[next_time][nonzero_index][next_opened_valves] = std::cmp::max(
                    dp[next_time][nonzero_index][next_opened_valves],
                    dp[time][location_index][opened_valves]
                        + (data[valve_index].flow * (max_time as i64 - next_time as i64)),
                );
                next_states.insert((next_time, nonzero_index, next_opened_valves));
            }
        }
    }
}

fn solve_part1(data: &[Valve]) -> i64 {
    let max_time = 30;

    let non_zero_valve_names: Vec<&str> = data
        .iter()
        .filter_map(|v| (v.flow > 0).then_some(v.name.as_str()))
        .collect();

    let mut dp = vec![
        vec![vec![0; 1 << non_zero_valve_names.len()]; non_zero_valve_names.len()];
        max_time + 1
    ];

    fill_dp_matrix(&mut dp, data, &non_zero_valve_names, max_time);

    dp.iter()
        .map(|slice| {
            slice
                .iter()
                .map(|slice| slice.iter().copied().max().unwrap())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn solve_part2(data: &[Valve]) -> i64 {
    let max_time = 26;

    let non_zero_valve_names: Vec<&str> = data
        .iter()
        .filter_map(|v| (v.flow > 0).then_some(v.name.as_str()))
        .collect();

    let mut dp = vec![
        vec![vec![0; 1 << non_zero_valve_names.len()]; non_zero_valve_names.len()];
        max_time + 1
    ];

    fill_dp_matrix(&mut dp, data, &non_zero_valve_names, max_time);

    let mut best_scores = vec![0; 1 << non_zero_valve_names.len()];
    #[allow(clippy::needless_range_loop)]
    for valve_pattern in 0..(1 << non_zero_valve_names.len()) {
        #[allow(clippy::needless_range_loop)]
        for time in 0..=max_time {
            for valve_location in 0..non_zero_valve_names.len() {
                best_scores[valve_pattern] = std::cmp::max(
                    best_scores[valve_pattern],
                    dp[time][valve_location][valve_pattern],
                );
            }
        }
    }

    let nonzero_best_scores = best_scores
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(pattern, score)| (score > 0).then_some((pattern, score)))
        .collect_vec();

    let mut overall_best = 0;
    for (pattern, score) in &nonzero_best_scores {
        for (other_pattern, other_score) in &nonzero_best_scores {
            if pattern & other_pattern != 0 {
                continue;
            }

            overall_best = std::cmp::max(overall_best, score + other_score);
        }
    }

    overall_best
}
