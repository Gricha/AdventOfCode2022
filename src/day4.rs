use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

type SectionRange = (u32, u32);

pub fn run_easy() {
    let result: u32 = read_input("inputs/day4.txt")
        .into_iter()
        .map(|ranges| {
            let val: (SectionRange, SectionRange) = ranges
                .split(',')
                .into_iter()
                .map(|x| -> SectionRange {
                    x.split('-')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            val
        })
        .map(|(x, y)| {
            if (x.0 <= y.0 && x.1 >= y.1) || (x.0 >= y.0 && x.1 <= y.1) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{}", result);
}

pub fn run_hard() {
    let result: u32 = read_input("inputs/day4.txt")
        .into_iter()
        .map(|ranges| {
            let val: (SectionRange, SectionRange) = ranges
                .split(',')
                .into_iter()
                .map(|x| -> SectionRange {
                    x.split('-')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            val
        })
        .map(|(x, y)| if (x.0 <= y.1 && y.0 <= x.1) { 1 } else { 0 })
        .sum();
    println!("{}", result);
}
