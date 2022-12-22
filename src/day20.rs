use std::collections::VecDeque;

use itertools::Itertools;

use crate::utils::read_input;

pub fn run_easy() {
    let original_order = read_input("inputs/day20.txt")
        .into_iter()
        .enumerate()
        .map(|(idx, val)| (idx, val.parse::<i64>().unwrap()))
        .collect_vec();

    assert!(original_order.iter().filter(|(x, y)| *x == 0).count() == 1);
    let zero_pos = original_order.iter().find_position(|x| x.1 == 0).unwrap().0;

    let numbers = original_order.len() as i64;

    let mut order = (0..numbers).into_iter().collect_vec();

    for (wh, i) in original_order.iter() {
        let idx = order.iter().find_position(|x| **x == *wh as i64).unwrap().0 as i64;
        let new_position = (idx + i).rem_euclid(numbers - 1);

        let val = order.remove(idx as usize);
        order.insert(new_position as usize, val);
    }

    let zero_idx = order
        .iter()
        .find_position(|x| **x == zero_pos.try_into().unwrap())
        .unwrap()
        .0;

    let val1 = original_order[order[(zero_idx + 1000) % numbers as usize] as usize].1;
    let val2 = original_order[order[(zero_idx + 2000) % numbers as usize] as usize].1;
    let val3 = original_order[order[(zero_idx + 3000) % numbers as usize] as usize].1;

    dbg!(val1 + val2 + val3);
}

pub fn run_hard() {
    let original_order = read_input("inputs/day20.txt")
        .into_iter()
        .enumerate()
        .map(|(idx, val)| (idx, val.parse::<i64>().unwrap() * 811589153))
        .collect_vec();

    assert!(original_order.iter().filter(|(x, y)| *x == 0).count() == 1);
    let zero_pos = original_order.iter().find_position(|x| x.1 == 0).unwrap().0;

    let numbers = original_order.len() as i64;

    let mut order = (0..numbers).into_iter().collect_vec();

    for _ in 0..10 {
        for (wh, i) in original_order.iter() {
            let idx = order.iter().find_position(|x| **x == *wh as i64).unwrap().0 as i64;
            let new_position = (idx + i).rem_euclid(numbers - 1);

            let val = order.remove(idx as usize);
            order.insert(new_position as usize, val);
        }
    }

    let zero_idx = order
        .iter()
        .find_position(|x| **x == zero_pos.try_into().unwrap())
        .unwrap()
        .0;

    let val1 = original_order[order[(zero_idx + 1000) % numbers as usize] as usize].1;
    let val2 = original_order[order[(zero_idx + 2000) % numbers as usize] as usize].1;
    let val3 = original_order[order[(zero_idx + 3000) % numbers as usize] as usize].1;

    dbg!(val1 + val2 + val3);
}
