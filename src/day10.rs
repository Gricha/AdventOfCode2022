use itertools::Itertools;

use crate::utils::read_input;

pub fn run_easy() {
    let input = read_input("inputs/day10.txt")
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.to_owned())
                .collect_vec()
        })
        .collect_vec();

    let mut cycle = 0;
    let mut values = [0; 1000];
    values[cycle] = 1;

    for command in input {
        if command[0].starts_with("noop") {
            cycle += 1;
            values[cycle] = values[cycle - 1];
        } else {
            let value_to_add = command[1].parse::<i32>().unwrap();
            values[cycle + 1] = values[cycle];
            values[cycle + 2] = values[cycle] + value_to_add;
            cycle += 2;
        }
    }

    let answer = values[19] * 20
        + values[59] * 60
        + values[99] * 100
        + values[139] * 140
        + values[179] * 180
        + values[219] * 220;

    println!("{}", answer);
}

pub fn run_hard() {
    let input = read_input("inputs/day10.txt")
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.to_owned())
                .collect_vec()
        })
        .collect_vec();

    let mut cycle = 0;
    let mut values: [i32; 250] = [0; 250];
    values[cycle] = 1;

    let mut crt = [[0; 40]; 6];

    for command in input {
        if command[0].starts_with("noop") {
            cycle += 1;
            values[cycle] = values[cycle - 1];
        } else {
            let value_to_add = command[1].parse::<i32>().unwrap();
            values[cycle + 1] = values[cycle];
            values[cycle + 2] = values[cycle] + value_to_add;
            cycle += 2;
        }
    }

    (0..240).for_each(|iter_cycle| {
        let position = iter_cycle % 40;
        let height = iter_cycle / 40;

        let cycle_value = values[iter_cycle];

        if ((cycle_value - 1)..=(cycle_value + 1)).contains(&(position as i32)) {
            crt[height][position] = 1;
        }
    });

    for i in 0..6 {
        for j in 0..40 {
            if crt[i][j] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
