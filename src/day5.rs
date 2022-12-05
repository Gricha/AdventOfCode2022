use crate::utils::read_input;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run_easy() {
    let input = read_input("inputs/day5.txt");
    let mut segments = input
        .split(|x| x.is_empty())
        .into_iter()
        .map(|x| x.iter().to_owned().collect_vec());

    let mut initial_state = segments
        .next()
        .unwrap()
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|x| {
            // floor
            let mut iter = x.chars();

            let mut value_mapping = HashMap::<u32, u8>::new();

            let mut column = 1;
            let mut idx = 0;
            loop {
                let val = iter.next();
                if let Some(character) = val {
                    if character.is_alphanumeric() {
                        value_mapping.insert(column, character as u8);
                    }
                    column = (idx / 4) + 1;
                    idx += 1;
                } else {
                    break;
                }
            }
            value_mapping
        })
        .rev()
        .fold(HashMap::new(), |mut acc, vals| {
            for (k, v) in vals.into_iter() {
                acc.entry(k).or_insert_with(Vec::new).push(v as char);
            }
            acc
        });
    let moves = segments
        .next()
        .unwrap()
        .into_iter()
        .map(|line| -> (u32, u32, u32) {
            line.split_ascii_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    for (how_many, from, to) in moves.iter() {
        initial_state
            .entry(*from)
            .or_insert_with(std::vec::Vec::new);
        initial_state.entry(*to).or_insert_with(std::vec::Vec::new);
        for _ in 0..*how_many {
            let val = initial_state.get_mut(from).unwrap().pop().unwrap();
            initial_state.get_mut(to).unwrap().push(val);
        }
    }

    let columns_count = initial_state.keys().len();

    for i in 1..=columns_count {
        print!(
            "{}",
            initial_state.get(&(i as u32)).unwrap().last().unwrap()
        );
    }
    println!();
}

pub fn run_hard() {
    let input = read_input("inputs/day5.txt");
    let mut segments = input
        .split(|x| x.is_empty())
        .into_iter()
        .map(|x| x.iter().to_owned().collect_vec());

    let mut initial_state = segments
        .next()
        .unwrap()
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|x| {
            // floor
            let mut iter = x.chars();

            let mut value_mapping = HashMap::<u32, u8>::new();

            let mut column = 1;
            let mut idx = 0;
            loop {
                let val = iter.next();
                if let Some(character) = val {
                    if character.is_alphanumeric() {
                        value_mapping.insert(column, character as u8);
                    }
                    column = (idx / 4) + 1;
                    idx += 1;
                } else {
                    break;
                }
            }
            value_mapping
        })
        .rev()
        .fold(HashMap::new(), |mut acc, vals| {
            for (k, v) in vals.into_iter() {
                acc.entry(k).or_insert_with(Vec::new).push(v as char);
            }
            acc
        });
    let moves = segments
        .next()
        .unwrap()
        .into_iter()
        .map(|line| -> (u32, u32, u32) {
            line.split_ascii_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    for (how_many, from, to) in moves.iter() {
        initial_state
            .entry(*from)
            .or_insert_with(std::vec::Vec::new);
        initial_state.entry(*to).or_insert_with(std::vec::Vec::new);

        let from_col = initial_state.get_mut(from).unwrap();
        let values = from_col
            .iter()
            .rev()
            .take(*how_many as usize)
            .rev()
            .cloned()
            .collect_vec();
        for _ in 0..*how_many {
            from_col.pop();
        }
        initial_state
            .get_mut(to)
            .unwrap()
            .extend(values.into_iter());
    }

    let columns_count = initial_state.keys().len();

    for i in 1..=columns_count {
        print!(
            "{}",
            initial_state.get(&(i as u32)).unwrap().last().unwrap()
        );
    }
    println!();
}
