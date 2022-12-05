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

            let mut column = 0;
            let mut idx = 0;
            loop {
                let val = iter.next();
                if let Some(character) = val {
                    if character != ' ' && character != '[' && character != ']' {
                        value_mapping.insert(column, character as u8);
                    }
                    column = idx / 4;
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
                acc.entry(k).or_insert_with(Vec::new);
                acc.get_mut(&k).unwrap().push(v as char);
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
            .entry(from - 1)
            .or_insert_with(std::vec::Vec::new);
        initial_state
            .entry(to - 1)
            .or_insert_with(std::vec::Vec::new);
        for _ in 0..*how_many {
            let val = initial_state.get_mut(&(from - 1)).unwrap().pop().unwrap();
            initial_state.get_mut(&(*to - 1)).unwrap().push(val);
        }
    }

    let columns_count = initial_state.keys().len();

    for i in 0..columns_count {
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

            let mut column = 0;
            let mut idx = 0;
            loop {
                let val = iter.next();
                if let Some(character) = val {
                    if character != ' ' && character != '[' && character != ']' {
                        value_mapping.insert(column, character as u8);
                    }
                    column = idx / 4;
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
                acc.entry(k).or_insert_with(Vec::new);
                acc.get_mut(&k).unwrap().push(v as char);
            }
            acc
        });
    dbg!(&initial_state);
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
            .entry(from - 1)
            .or_insert_with(std::vec::Vec::new);
        initial_state
            .entry(to - 1)
            .or_insert_with(std::vec::Vec::new);

        let from_col = initial_state.get_mut(&(from - 1)).unwrap();
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
            .get_mut(&(to - 1))
            .unwrap()
            .extend(values.into_iter());
    }

    let columns_count = initial_state.keys().len();

    for i in 0..columns_count {
        print!(
            "{}",
            initial_state.get(&(i as u32)).unwrap().last().unwrap()
        );
    }
    println!();
}
