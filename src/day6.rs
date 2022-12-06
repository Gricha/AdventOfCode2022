use std::collections::{HashSet, VecDeque};

use crate::utils::read_input;

pub fn run_easy() {
    let input = read_input("inputs/day6.txt").into_iter().next().unwrap();

    let mut items = VecDeque::new();
    for c in input.chars().take(4) {
        items.push_front(c);
    }

    for (i, c) in input.chars().skip(4).enumerate() {
        let count = HashSet::<char>::from_iter(items.clone().into_iter()).len();
        if count == 4 {
            println!("{}", i + 4);
            return;
        }
        items.pop_back();
        items.push_front(c);
    }
}

pub fn run_hard() {
    let input = read_input("inputs/day6.txt").into_iter().next().unwrap();

    let mut items = VecDeque::new();
    for c in input.chars().take(14) {
        items.push_front(c);
    }

    for (i, c) in input.chars().skip(14).enumerate() {
        let count = HashSet::<char>::from_iter(items.clone().into_iter()).len();
        if count == 14 {
            println!("{}", i + 14);
            return;
        }
        items.pop_back();
        items.push_front(c);
    }
}
