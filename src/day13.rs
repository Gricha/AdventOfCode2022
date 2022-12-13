use std::cmp::Ordering;

use itertools::Itertools;

use crate::utils::read_input;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ElfNumber {
    Int(i32),
    List(Vec<ElfNumber>),
}

impl Ord for ElfNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            ElfNumber::Int(v) => match other {
                ElfNumber::Int(k) => v.cmp(k),
                ElfNumber::List(vv) => {
                    ElfNumber::List(vec![ElfNumber::Int(*v)]).cmp(&ElfNumber::List(vv.to_vec()))
                }
            },
            ElfNumber::List(l) => match other {
                ElfNumber::Int(k) => {
                    ElfNumber::List(l.to_vec()).cmp(&ElfNumber::List(vec![ElfNumber::Int(*k)]))
                }
                ElfNumber::List(ol) => {
                    let len_left = l.len();
                    let len_right = ol.len();

                    let mut idx = 0;
                    loop {
                        if idx < len_left && idx < len_right {
                            //both have values
                            let cmp_result = l[idx].cmp(&ol[idx]);
                            if cmp_result != Ordering::Equal {
                                return cmp_result;
                            } else {
                                idx += 1;
                                continue;
                            }
                        }

                        if idx >= len_left && idx < len_right {
                            return Ordering::Less;
                        }

                        if idx < len_left && idx >= len_right {
                            return Ordering::Greater;
                        }

                        if idx >= len_left && idx >= len_right {
                            return Ordering::Equal;
                        }
                    }
                }
            },
        }
    }
}

impl PartialOrd for ElfNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_number(start_index: usize, n: &[char]) -> (ElfNumber, usize) {
    assert!(n[start_index] == '[');

    let mut idx = start_index + 1;

    let mut numbers = vec![];

    let mut has_number = false;
    let mut current_number = 0;

    loop {
        match n[idx] {
            '[' => {
                let (num, end_idx) = read_number(idx, n);
                numbers.push(num);
                has_number = false;
                idx = end_idx;
            }
            ']' => {
                if has_number {
                    numbers.push(ElfNumber::Int(current_number));
                }
                return (ElfNumber::List(numbers), idx);
            }
            ',' => {
                if has_number {
                    numbers.push(ElfNumber::Int(current_number));
                    has_number = false;
                }
                current_number = 0;
            }
            _ => {
                has_number = true;
                current_number *= 10;
                current_number += ((n[idx] as u8) - b'0') as i32;
            }
        }
        idx += 1;
    }
}

pub fn run_easy() {
    let result: u32 = read_input("inputs/day13.txt")
        .chunks(3)
        .into_iter()
        .map(|x| {
            x.iter()
                .filter_map(|v| {
                    if v.is_empty() {
                        return None;
                    }
                    let chars = v.chars().collect_vec();
                    Some(read_number(0, &chars).0)
                })
                .collect_vec()
        })
        .enumerate()
        .map(
            |(idx, vals)| {
                if vals[0] < vals[1] {
                    idx as u32 + 1
                } else {
                    0
                }
            },
        )
        .sum();

    println!("{}", result);
}

pub fn run_hard() {
    let input = read_input("inputs/day13.txt")
        .into_iter()
        .filter_map(|v| {
            if v.is_empty() {
                return None;
            }
            let chars = v.chars().collect_vec();
            Some(read_number(0, &chars).0)
        })
        .sorted()
        .collect_vec();

    let idx1 = input
        .iter()
        .find_position(|x| **x == ElfNumber::List(vec![ElfNumber::List(vec![ElfNumber::Int(2)])]))
        .unwrap()
        .0
        + 1;
    let idx2 = input
        .iter()
        .find_position(|x| **x == ElfNumber::List(vec![ElfNumber::List(vec![ElfNumber::Int(6)])]))
        .unwrap()
        .0
        + 1;

    println!("{}", idx1 * idx2);
}
