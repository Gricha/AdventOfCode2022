use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::utils::read_input;

const SIZE: usize = 24;

pub fn run_easy() {
    let input = read_input("inputs/day18.txt")
        .into_iter()
        .map(|x| -> (i32, i32, i32) {
            x.split(',')
                .map(|c| c.parse::<i32>().unwrap() + 1)
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut map = [[[0; SIZE]; SIZE]; SIZE];
    for (x, y, z) in input {
        map[x as usize][y as usize][z as usize] = 1;
    }

    let mut exposed_sides = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            for k in 0..SIZE {
                if map[i][j][k] == 1 {
                    if map[i - 1][j][k] == 0 {
                        exposed_sides += 1;
                    }
                    if map[i + 1][j][k] == 0 {
                        exposed_sides += 1;
                    }

                    if map[i][j - 1][k] == 0 {
                        exposed_sides += 1;
                    }
                    if map[i][j + 1][k] == 0 {
                        exposed_sides += 1;
                    }

                    if map[i][j][k - 1] == 0 {
                        exposed_sides += 1;
                    }
                    if map[i][j][k + 1] == 0 {
                        exposed_sides += 1;
                    }
                }
            }
        }
    }
    println!("{}", exposed_sides);
}

pub fn run_hard() {
    let input = read_input("inputs/day18.txt")
        .into_iter()
        .map(|x| -> (i32, i32, i32) {
            x.split(',')
                .map(|c| c.parse::<i32>().unwrap() + 1)
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut map = [[[0; SIZE]; SIZE]; SIZE];
    for (x, y, z) in input {
        map[x as usize][y as usize][z as usize] = 1;
    }

    let mut color_queue = VecDeque::new();
    color_queue.push_back((1, 1, 1));

    while !color_queue.is_empty() {
        let (x, y, z) = color_queue.pop_front().unwrap();

        if map[x][y][z] == 0 {
            map[x][y][z] = 2;

            if x > 0 && map[x - 1][y][z] == 0 {
                color_queue.push_back((x - 1, y, z));
            }
            if x < SIZE - 1 && map[x + 1][y][z] == 0 {
                color_queue.push_back((x + 1, y, z));
            }
            if y > 0 && map[x][y - 1][z] == 0 {
                color_queue.push_back((x, y - 1, z));
            }
            if y < SIZE - 1 && map[x][y + 1][z] == 0 {
                color_queue.push_back((x, y + 1, z));
            }
            if z > 0 && map[x][y][z - 1] == 0 {
                color_queue.push_back((x, y, z - 1));
            }
            if z < SIZE - 1 && map[x][y][z + 1] == 0 {
                color_queue.push_back((x, y, z + 1));
            }
        }
    }

    let mut exposed_sides = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            for k in 0..SIZE {
                if map[i][j][k] == 1 {
                    if map[i - 1][j][k] == 2 {
                        exposed_sides += 1;
                    }
                    if map[i + 1][j][k] == 2 {
                        exposed_sides += 1;
                    }

                    if map[i][j - 1][k] == 2 {
                        exposed_sides += 1;
                    }
                    if map[i][j + 1][k] == 2 {
                        exposed_sides += 1;
                    }

                    if map[i][j][k - 1] == 2 {
                        exposed_sides += 1;
                    }
                    if map[i][j][k + 1] == 2 {
                        exposed_sides += 1;
                    }
                }
            }
        }
    }

    println!("{}", exposed_sides);
}
