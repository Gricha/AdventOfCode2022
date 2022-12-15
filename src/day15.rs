use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use itertools::Itertools;
use regex::Regex;

use crate::utils::read_input;

// return if performed a fix
fn fix_range_vector(range_vector: &mut Vec<RangeInclusive<i64>>) -> bool {
    for i in 0..(range_vector.len() - 1) {
        if range_vector[i].start() <= range_vector[i + 1].end()
            && range_vector[i].end() >= range_vector[i + 1].start()
        {
            // found intersection
            range_vector[i] =
                (std::cmp::min(*range_vector[i].start(), *range_vector[i + 1].start()))
                    ..=(std::cmp::max(*range_vector[i].end(), *range_vector[i + 1].end()));
            range_vector.remove(i + 1);
            return true;
        }

        // another option is they are adjacent
        if range_vector[i].end() + 1 == *range_vector[i + 1].start() {
            range_vector[i] =
                (std::cmp::min(*range_vector[i].start(), *range_vector[i + 1].start()))
                    ..=(std::cmp::max(*range_vector[i].end(), *range_vector[i + 1].end()));
            range_vector.remove(i + 1);
            return true;
        }
    }
    false
}

pub fn run_easy() {
    // return;
    let input = read_input("inputs/day15.txt")
        .into_iter()
        .map(|line| {
            let (left, right): (&str, &str) = line
                .split(": closest beacon is at ")
                .collect_tuple()
                .unwrap();

            let regex = Regex::new(r"Sensor at x=(.*?), y=(.*)").unwrap();
            let captures_left = regex.captures(left).unwrap();

            let x_left = captures_left
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            let y_left = captures_left
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();

            let regex_right = Regex::new(r"x=(.*?), y=(.*)").unwrap();
            let captures_right = regex_right.captures(right).unwrap();
            let x_right = captures_right
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            let y_right = captures_right
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();

            ((x_left, y_left), (x_right, y_right))
        })
        .collect_vec();

    let row = 2000000;
    // let row = 10;

    let mut beacon_spot_on_row = HashSet::<i64>::new();

    let mut ranges_at_row = Vec::<RangeInclusive<i64>>::new();

    for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in input {
        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        let distance_to_designated_row = (sensor_y - row).abs();

        if distance >= distance_to_designated_row {
            let delta = distance - distance_to_designated_row;

            let range = (sensor_x - delta)..=(sensor_x + delta);

            if ranges_at_row.is_empty() {
                ranges_at_row.push(range);
            } else {
                let mut added = false;
                for i in 0..ranges_at_row.len() {
                    if ranges_at_row[i].start() <= range.end()
                        && ranges_at_row[i].end() >= range.start()
                    {
                        // they intersect

                        ranges_at_row[i] =
                            (std::cmp::min(*range.start(), *ranges_at_row[i].start()))
                                ..=(std::cmp::max(*range.end(), *ranges_at_row[i].end()));
                        added = true;
                        break;
                    }
                }

                if !added {
                    ranges_at_row.push(range);
                }

                // after molding a new range, more ranges may intersect
                loop {
                    let fixed = fix_range_vector(&mut ranges_at_row);
                    if !fixed {
                        break;
                    }
                }
            }
        }

        if beacon_y == row {
            beacon_spot_on_row.insert(beacon_x);
        }
    }

    let sum: i32 = ranges_at_row.into_iter().map(|r| r.count() as i32).sum();

    println!("{}", sum - beacon_spot_on_row.len() as i32);
}

pub fn run_hard() {
    println!("skipping since it runs for 5 seconds");
    return;
    let input = read_input("inputs/day15.txt")
        .into_iter()
        .map(|line| {
            let (left, right): (&str, &str) = line
                .split(": closest beacon is at ")
                .collect_tuple()
                .unwrap();

            let regex = Regex::new(r"Sensor at x=(.*?), y=(.*)").unwrap();
            let captures_left = regex.captures(left).unwrap();

            let x_left = captures_left
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            let y_left = captures_left
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();

            let regex_right = Regex::new(r"x=(.*?), y=(.*)").unwrap();
            let captures_right = regex_right.captures(right).unwrap();
            let x_right = captures_right
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();
            let y_right = captures_right
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();

            ((x_left, y_left), (x_right, y_right))
        })
        .collect_vec();

    let mut range_rows = HashMap::<i64, Vec<RangeInclusive<i64>>>::new();
    let max_boundary = 4000000;

    for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in input {
        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        for delta_vertical in 0..=distance {
            let delta_horizontal = distance - delta_vertical;
            let range = (sensor_x - delta_horizontal)..=(sensor_x + delta_horizontal);
            for delta_dir in [sensor_y - delta_vertical, sensor_y + delta_vertical] {
                if delta_dir < 0 || delta_dir > max_boundary {
                    continue;
                }
                range_rows
                    .entry(delta_dir)
                    .or_insert_with(std::vec::Vec::new);
                let ranges_at_row: &mut Vec<RangeInclusive<i64>> =
                    range_rows.get_mut(&delta_dir).unwrap();

                if ranges_at_row.is_empty() {
                    ranges_at_row.push(range.clone());
                } else {
                    let mut added = false;
                    for i in 0..ranges_at_row.len() {
                        if ranges_at_row[i].start() <= range.end()
                            && ranges_at_row[i].end() >= range.start()
                        {
                            // they intersect

                            ranges_at_row[i] =
                                (std::cmp::min(*range.start(), *ranges_at_row[i].start()))
                                    ..=(std::cmp::max(*range.end(), *ranges_at_row[i].end()));
                            added = true;
                            break;
                        }
                        // check adjacency
                        if *ranges_at_row[i].start() == range.end() + 1
                            || *ranges_at_row[i].end() == range.start() - 1
                        {
                            ranges_at_row[i] =
                                (std::cmp::min(*range.start(), *ranges_at_row[i].start()))
                                    ..=(std::cmp::max(*range.end(), *ranges_at_row[i].end()));
                            added = true;
                            break;
                        }
                    }

                    if !added {
                        // add it in a proper spot
                        if range.end() < ranges_at_row[0].start() {
                            ranges_at_row.insert(0, range.clone())
                        } else {
                            ranges_at_row.push(range.clone());
                        }
                    }

                    // after molding a new range, more ranges may intersect
                    loop {
                        let fixed = fix_range_vector(ranges_at_row);
                        if !fixed {
                            break;
                        }
                    }
                }
            }
        }
    }

    for (k, v) in range_rows {
        if v.len() > 1 {
            println!("{}", k + 4000000 * (v[0].end() + 1));
            return;
        }
    }
}
