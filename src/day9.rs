use std::collections::BTreeSet;

use itertools::Itertools;

use crate::utils::read_input;

fn touching(a: (i32, i32), b: (i32, i32)) -> bool {
    if (b.0 - a.0).abs() > 1 || (b.1 - a.1).abs() > 1 {
        return false;
    }
    true
}

fn move_tail(h: (i32, i32), t: &mut (i32, i32)) {
    if touching(h, *t) {
        return;
    } else {
        if h.0 > t.0 {
            t.0 += 1;
        } else if h.0 < t.0 {
            t.0 -= 1;
        }

        if h.1 > t.1 {
            t.1 += 1;
        } else if h.1 < t.1 {
            t.1 -= 1;
        }
    }
}

pub fn run_easy() {
    let input = read_input("inputs/day9.txt")
        .into_iter()
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let val = line[2..].parse::<u32>().unwrap();
            (dir, val)
        })
        .collect_vec();

    let mut tail_position_set: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    tail_position_set.insert(tail_position);

    for (dir, steps) in input.into_iter() {
        for _ in 0..steps {
            match dir {
                'R' => {
                    head_position.0 += 1;
                }
                'L' => {
                    head_position.0 -= 1;
                }
                'U' => {
                    head_position.1 += 1;
                }
                'D' => {
                    head_position.1 -= 1;
                }
                _ => unreachable!(),
            }
            move_tail(head_position, &mut tail_position);
            tail_position_set.insert(tail_position);
        }
    }
    println!("{}", tail_position_set.len());
}

pub fn run_hard() {
    let input = read_input("inputs/day9.txt")
        .into_iter()
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let val = line[2..].parse::<u32>().unwrap();
            (dir, val)
        })
        .collect_vec();

    let mut tail_position_set: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut rope_positions = [(0, 0); 10];
    tail_position_set.insert((0, 0));

    for (dir, steps) in input.into_iter() {
        for _ in 0..steps {
            match dir {
                'R' => {
                    rope_positions[0].0 += 1;
                }
                'L' => {
                    rope_positions[0].0 -= 1;
                }
                'U' => {
                    rope_positions[0].1 += 1;
                }
                'D' => {
                    rope_positions[0].1 -= 1;
                }
                _ => unreachable!(),
            }
            for i in 0..9 {
                move_tail(rope_positions[i], &mut rope_positions[i + 1]);
            }
            tail_position_set.insert(rope_positions[9]);
        }
    }
    println!("{}", tail_position_set.len());
}
