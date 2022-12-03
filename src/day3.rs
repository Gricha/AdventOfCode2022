use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

pub fn run_easy() {
    let input: u32 = read_input("inputs/day3.txt")
        .into_iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| {
                    let val = c as u8;
                    if (b'a'..=b'z').contains(&val) {
                        val - b'a' + 1
                    } else {
                        val - b'A' + 27
                    }
                })
                .collect_vec()
        })
        .map(|sack| {
            let len = sack.len();
            let sacks: (HashSet<u8>, HashSet<u8>) = sack
                .chunks(len / 2)
                .map(|v| HashSet::<u8>::from_iter(v.iter().cloned()))
                .collect_tuple()
                .unwrap();
            let intersection = sacks
                .0
                .intersection(&sacks.1)
                .next()
                .expect("must intersect at least one");
            *intersection as u32
        })
        .sum();

    println!("{}", input);
}

pub fn run_hard() {
    let input: u32 = read_input("inputs/day3.txt")
        .into_iter()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| {
                    let val = c as u8;
                    if (b'a'..=b'z').contains(&val) {
                        val - b'a' + 1
                    } else {
                        val - b'A' + 27
                    }
                })
                .collect_vec()
        })
        .chunks(3)
        .into_iter()
        .map(|sack| {
            let sacks: (HashSet<u8>, HashSet<u8>, HashSet<u8>) = sack
                .map(|v| HashSet::<u8>::from_iter(v.iter().cloned()))
                .collect_tuple()
                .unwrap();
            let intersect_1_2 = sacks
                .0
                .intersection(&sacks.1)
                .into_iter()
                .cloned()
                .collect::<HashSet<u8>>();

            let intersect = intersect_1_2
                .intersection(&sacks.2)
                .next()
                .expect("must intersect at least one");
            *intersect as u32
        })
        .sum();

    println!("{}", input);
}
