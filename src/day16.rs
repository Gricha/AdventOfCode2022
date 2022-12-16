use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

use crate::utils::read_input;

fn max_path(
    point: &str,
    flows: &HashMap<String, i32>,
    graph: &HashMap<String, Vec<String>>,
    yield_cache: &mut HashMap<(String, String, i32), i32>,
    flows_opened: HashSet<String>,
    minutes_left: i32,
    flow_per_minute: i32,
    flow_so_far: i32,
) -> i32 {
    if minutes_left == 0 {
        return flow_so_far;
    }

    let string_of_flows_opened_yet = flows_opened.clone().into_iter().sorted().join(",");

    let mut max_value = flow_so_far;

    if let Some(value) = yield_cache.get(&(
        point.to_string(),
        string_of_flows_opened_yet.to_string(),
        minutes_left,
    )) {
        return max_value + value;
    }

    // Did we unlock current one?
    if !flows_opened.contains(point) && (*flows.get(point).unwrap() > 0) {
        // let's open it and see if it was worth it
        let new_flows_so_far = flow_so_far + flow_per_minute;
        let new_flow_per_minute = flow_per_minute + flows.get(point).unwrap();
        let mut new_flows_opened = flows_opened.clone();
        new_flows_opened.insert(point.to_string());

        max_value = std::cmp::max(
            max_value,
            max_path(
                point,
                flows,
                graph,
                yield_cache,
                new_flows_opened,
                minutes_left - 1,
                new_flow_per_minute,
                new_flows_so_far,
            ),
        );
    }

    // separately what if we go random ways..
    for potential_path in graph.get(point).unwrap() {
        max_value = std::cmp::max(
            max_value,
            max_path(
                potential_path,
                flows,
                graph,
                yield_cache,
                flows_opened.clone(),
                minutes_left - 1,
                flow_per_minute,
                flow_so_far + flow_per_minute,
            ),
        );
    }

    let max_added_yield = max_value - flow_so_far;
    yield_cache.insert(
        (point.to_string(), string_of_flows_opened_yet, minutes_left),
        max_added_yield,
    );

    max_value
}

pub fn run_easy() {
    let input = read_input("inputs/day16.txt")
        .into_iter()
        .map(|line| {
            let regex =
                Regex::new(r"Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                    .unwrap();

            let captures = regex.captures(&line).unwrap();

            let valve_id = captures.get(1).unwrap().as_str().to_owned();
            let valve_rate = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let ways_out = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(',')
                .map(|x| x.trim().to_owned())
                .collect_vec();
            (valve_id, valve_rate, ways_out)
        })
        .collect_vec();

    let mut graph = HashMap::new();
    let mut flows = HashMap::new();

    for (valve, strength, ways) in input {
        graph.insert(valve.to_string(), ways);
        flows.insert(valve, strength);
    }

    let mut yield_cache = HashMap::new();

    let max_value = max_path(
        "AA",
        &flows,
        &graph,
        &mut yield_cache,
        HashSet::new(),
        30,
        0,
        0,
    );

    println!("{}", max_value);
}

fn max_path_with_restriction(
    point: &str,
    flows: &HashMap<String, i32>,
    graph: &HashMap<String, Vec<String>>,
    restricted_valves: &HashSet<String>,
    yield_cache: &mut HashMap<(String, String, i32), i32>,
    flows_opened: HashSet<String>,
    minutes_left: i32,
    flow_per_minute: i32,
    flow_so_far: i32,
) -> i32 {
    if minutes_left == 0 {
        return flow_so_far;
    }

    let string_of_flows_opened_yet = flows_opened.clone().into_iter().sorted().join(",");

    let mut max_value = flow_so_far;

    if let Some(value) = yield_cache.get(&(
        point.to_string(),
        string_of_flows_opened_yet.to_string(),
        minutes_left,
    )) {
        return max_value + value;
    }

    // Did we unlock current one?
    if !flows_opened.contains(point)
        && !restricted_valves.contains(point)
        && (*flows.get(point).unwrap() > 0)
    {
        // let's open it and see if it was worth it
        let new_flows_so_far = flow_so_far + flow_per_minute;
        let new_flow_per_minute = flow_per_minute + flows.get(point).unwrap();
        let mut new_flows_opened = flows_opened.clone();
        new_flows_opened.insert(point.to_string());

        max_value = std::cmp::max(
            max_value,
            max_path_with_restriction(
                point,
                flows,
                graph,
                restricted_valves,
                yield_cache,
                new_flows_opened,
                minutes_left - 1,
                new_flow_per_minute,
                new_flows_so_far,
            ),
        );
    }

    // separately what if we go random ways..
    for potential_path in graph.get(point).unwrap() {
        max_value = std::cmp::max(
            max_value,
            max_path_with_restriction(
                potential_path,
                flows,
                graph,
                restricted_valves,
                yield_cache,
                flows_opened.clone(),
                minutes_left - 1,
                flow_per_minute,
                flow_so_far + flow_per_minute,
            ),
        );
    }

    let max_added_yield = max_value - flow_so_far;
    yield_cache.insert(
        (point.to_string(), string_of_flows_opened_yet, minutes_left),
        max_added_yield,
    );

    max_value
}

pub fn run_hard() {
    let input = read_input("inputs/day16.txt")
        .into_iter()
        .map(|line| {
            let regex =
                Regex::new(r"Valve (.*) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                    .unwrap();

            let captures = regex.captures(&line).unwrap();

            let valve_id = captures.get(1).unwrap().as_str().to_owned();
            let valve_rate = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let ways_out = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(',')
                .map(|x| x.trim().to_owned())
                .collect_vec();
            (valve_id, valve_rate, ways_out)
        })
        .collect_vec();

    let mut graph = HashMap::new();
    let mut flows = HashMap::new();

    for (valve, strength, ways) in input {
        graph.insert(valve.to_string(), ways);
        flows.insert(valve, strength);
    }

    let keyspace_size = flows.iter().filter(|(k, v)| **v > 0).count();
    let keys = flows
        .iter()
        .filter(|(k, v)| **v > 0)
        .map(|(k, v)| k.to_string())
        .sorted()
        .collect_vec();

    // let mut cache_reverse = HashMap::<String, i32>::new();

    let max_value_boundary = i32::pow(2, keyspace_size as u32 - 1);
    let value_split = max_value_boundary / 8;

    let mut ranges = vec![];

    for i in 0..8 {
        ranges.push((i * value_split as usize)..((i + 1) * value_split as usize));
    }
    println!("{:?}", ranges);

    let max_glb = ranges
        .par_iter()
        .map(|range| {
            let mut max_global = 0;
            for i in range.clone() {
                let mut bitwise_number = i;
                let mut set_zero = HashSet::new();
                let mut set_one = HashSet::new();
                for j in 0..keyspace_size {
                    if bitwise_number % 2 == 0 {
                        set_zero.insert(keys[j].to_string());
                    } else {
                        set_one.insert(keys[j].to_string());
                    }
                    bitwise_number /= 2;
                }

                let mut zero_yield_cache = HashMap::new();
                let zero_value = max_path_with_restriction(
                    "AA",
                    &flows,
                    &graph,
                    &set_zero,
                    &mut zero_yield_cache,
                    HashSet::new(),
                    26,
                    0,
                    0,
                );
                let mut one_yield_cache = HashMap::new();
                let one_value = max_path_with_restriction(
                    "AA",
                    &flows,
                    &graph,
                    &set_one,
                    &mut one_yield_cache,
                    HashSet::new(),
                    26,
                    0,
                    0,
                );

                max_global = std::cmp::max(max_global, zero_value + one_value);
            }
            max_global
        })
        .max()
        .unwrap();

    println!("{}", max_glb);
}
