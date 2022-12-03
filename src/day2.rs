use itertools::Itertools;

use crate::utils::read_input;

pub fn run_easy() {
    let input = read_input("inputs/day2.txt");
    let res: u32 = input
        .into_iter()
        .map(|l| l.split(' ').map(|v| v.to_string()).collect_vec())
        .map(|x| {
            let mut i = x.into_iter();
            (
                i.next().unwrap().chars().into_iter().next().unwrap(),
                i.next().unwrap().chars().into_iter().next().unwrap(),
            )
        })
        .map(|(their, mine)| -> u32 {
            let price = mine as u8 - b'X' + 1;
            match their {
                'A' => match mine {
                    'X' => 3 + price,
                    'Y' => 6 + price,
                    'Z' => price,
                    _ => unreachable!(),
                },
                'B' => match mine {
                    'X' => price,
                    'Y' => 3 + price,
                    'Z' => 6 + price,
                    _ => unreachable!(),
                },
                'C' => match mine {
                    'X' => 6 + price,
                    'Y' => price,
                    'Z' => 3 + price,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
            .into()
        })
        .sum();

    println!("{}", res);
}

pub fn run_hard() {
    let input = read_input("inputs/day2.txt");
    let res: u32 = input
        .into_iter()
        .map(|l| l.split(' ').map(|v| v.to_string()).collect_vec())
        .map(|x| {
            let mut i = x.into_iter();
            (
                i.next().unwrap().chars().into_iter().next().unwrap(),
                i.next().unwrap().chars().into_iter().next().unwrap(),
            )
        })
        .map(|(their, mine)| -> u32 {
            let outcome = (mine as u8 - b'X') * 3;
            match their {
                'A' => match mine {
                    'X' => 3 + outcome,
                    'Y' => 1 + outcome,
                    'Z' => 2 + outcome,
                    _ => unreachable!(),
                },
                'B' => match mine {
                    'X' => 1 + outcome,
                    'Y' => 2 + outcome,
                    'Z' => 3 + outcome,
                    _ => unreachable!(),
                },
                'C' => match mine {
                    'X' => 2 + outcome,
                    'Y' => 3 + outcome,
                    'Z' => 1 + outcome,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
            .into()
        })
        .sum();

    println!("{}", res);
}
