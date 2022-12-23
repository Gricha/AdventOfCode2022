use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use crate::utils::read_input;

const SIZE: usize = 1000;
const OFFSET: usize = 500;

pub fn run_easy() {
    let input = read_input("inputs/day23.txt");

    let mut board = [[0; SIZE]; SIZE];

    let mut elves = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            match character {
                '.' => board[x + OFFSET][y + OFFSET] = 0,
                '#' => {
                    board[x + OFFSET][y + OFFSET] = 1;
                    elves.push((x + OFFSET, y + OFFSET));
                }
                _ => unreachable!(),
            }
        }
    }

    let mut directions = VecDeque::from_iter(vec!["N", "S", "W", "E"].into_iter());

    let mut num = 0;
    loop {
        num += 1;
        if num > 10 {
            break;
        }
        // map from destination to elf who wants to go there
        let mut proposed_moves = HashMap::<(usize, usize), (usize, usize)>::new();
        let mut keys_to_remove = HashSet::new();

        for elf in elves.iter() {
            let mut proposed_move = None;

            // should elf move at all?
            if board[elf.0 - 1][elf.1 - 1]
                + board[elf.0][elf.1 - 1]
                + board[elf.0 + 1][elf.1 - 1]
                + board[elf.0 + 1][elf.1]
                + board[elf.0 + 1][elf.1 + 1]
                + board[elf.0][elf.1 + 1]
                + board[elf.0 - 1][elf.1 + 1]
                + board[elf.0 - 1][elf.1]
                == 0
            {
                continue;
            }

            for dir in directions.iter() {
                match *dir {
                    "N" => {
                        if board[elf.0 - 1][elf.1 - 1]
                            + board[elf.0][elf.1 - 1]
                            + board[elf.0 + 1][elf.1 - 1]
                            == 0
                        {
                            proposed_move = Some((elf.0, elf.1 - 1));
                            break;
                        }
                    }
                    "S" => {
                        if board[elf.0 - 1][elf.1 + 1]
                            + board[elf.0][elf.1 + 1]
                            + board[elf.0 + 1][elf.1 + 1]
                            == 0
                        {
                            proposed_move = Some((elf.0, elf.1 + 1));
                            break;
                        }
                    }
                    "W" => {
                        if board[elf.0 - 1][elf.1 - 1]
                            + board[elf.0 - 1][elf.1]
                            + board[elf.0 - 1][elf.1 + 1]
                            == 0
                        {
                            proposed_move = Some((elf.0 - 1, elf.1));
                            break;
                        }
                    }
                    "E" => {
                        if board[elf.0 + 1][elf.1 - 1]
                            + board[elf.0 + 1][elf.1]
                            + board[elf.0 + 1][elf.1 + 1]
                            == 0
                        {
                            proposed_move = Some((elf.0 + 1, elf.1));
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if let Some(mv) = proposed_move {
                if let Entry::Vacant(e) = proposed_moves.entry(mv) {
                    e.insert(*elf);
                } else {
                    keys_to_remove.insert(mv);
                }
            }
        }

        for k in keys_to_remove.into_iter() {
            proposed_moves.remove(&k);
        }

        if proposed_moves.is_empty() {
            break;
        }

        let mut new_elves = HashMap::new();

        for (k, v) in proposed_moves.iter() {
            board[k.0][k.1] = 1;
            board[v.0][v.1] = 0;
            new_elves.insert(*v, *k);
        }

        for elf in elves.iter_mut() {
            if let Some(new_pos) = new_elves.get(elf) {
                elf.0 = new_pos.0;
                elf.1 = new_pos.1;
            }
        }

        // rotate directions
        let nd = directions.pop_front().unwrap();
        directions.push_back(nd);
    }
    let mut horizontal_span = (usize::MAX, 0);
    let mut vertical_span = (usize::MAX, 0);

    for y in 0..SIZE {
        for x in 0..SIZE {
            if board[x][y] == 1 {
                horizontal_span = (
                    std::cmp::min(horizontal_span.0, x),
                    std::cmp::max(horizontal_span.1, x),
                );
                vertical_span = (
                    std::cmp::min(vertical_span.0, y),
                    std::cmp::max(vertical_span.1, y),
                );
            }
        }
    }

    let width = horizontal_span.1 - horizontal_span.0 + 1;
    let height = vertical_span.1 - vertical_span.0 + 1;

    dbg!(width * height - elves.len());
}

pub fn run_hard() {
    let input = read_input("inputs/day23.txt");

    let mut board = [[0; SIZE]; SIZE];

    let mut elves = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            match character {
                '.' => board[x + OFFSET][y + OFFSET] = 0,
                '#' => {
                    board[x + OFFSET][y + OFFSET] = 1;
                    elves.push((x + OFFSET, y + OFFSET));
                }
                _ => unreachable!(),
            }
        }
    }

    let mut directions = VecDeque::from_iter(vec!["N", "S", "W", "E"].into_iter());

    let mut num = 0;
    loop {
        num += 1;
        // map from destination to elf who wants to go there
        let mut proposed_moves = HashMap::<(usize, usize), (usize, usize)>::new();
        let mut keys_to_remove = HashSet::new();

        for elf in elves.iter() {
            let mut proposed_move = None;

            // should elf move at all?
            if board[elf.0 - 1][elf.1 - 1]
                + board[elf.0][elf.1 - 1]
                + board[elf.0 + 1][elf.1 - 1]
                + board[elf.0 + 1][elf.1]
                + board[elf.0 + 1][elf.1 + 1]
                + board[elf.0][elf.1 + 1]
                + board[elf.0 - 1][elf.1 + 1]
                + board[elf.0 - 1][elf.1]
                == 0
            {
                continue;
            }

            for dir in directions.iter() {
                match *dir {
                    "N" => {
                        if board[elf.0 - 1][elf.1 - 1]
                            + board[elf.0][elf.1 - 1]
                            + board[elf.0 + 1][elf.1 - 1]
                            == 0
                        {
                            proposed_move = Some((elf.0, elf.1 - 1));
                            break;
                        }
                    }
                    "S" => {
                        if board[elf.0 - 1][elf.1 + 1]
                            + board[elf.0][elf.1 + 1]
                            + board[elf.0 + 1][elf.1 + 1]
                            == 0
                        {
                            proposed_move = Some((elf.0, elf.1 + 1));
                            break;
                        }
                    }
                    "W" => {
                        if board[elf.0 - 1][elf.1 - 1]
                            + board[elf.0 - 1][elf.1]
                            + board[elf.0 - 1][elf.1 + 1]
                            == 0
                        {
                            proposed_move = Some((elf.0 - 1, elf.1));
                            break;
                        }
                    }
                    "E" => {
                        if board[elf.0 + 1][elf.1 - 1]
                            + board[elf.0 + 1][elf.1]
                            + board[elf.0 + 1][elf.1 + 1]
                            == 0
                        {
                            proposed_move = Some((elf.0 + 1, elf.1));
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if let Some(mv) = proposed_move {
                if let Entry::Vacant(e) = proposed_moves.entry(mv) {
                    e.insert(*elf);
                } else {
                    keys_to_remove.insert(mv);
                }
            }
        }

        for k in keys_to_remove.into_iter() {
            proposed_moves.remove(&k);
        }

        if proposed_moves.is_empty() {
            break;
        }

        let mut new_elves = HashMap::new();

        for (k, v) in proposed_moves.iter() {
            board[k.0][k.1] = 1;
            board[v.0][v.1] = 0;
            new_elves.insert(*v, *k);
        }

        for elf in elves.iter_mut() {
            if let Some(new_pos) = new_elves.get(elf) {
                elf.0 = new_pos.0;
                elf.1 = new_pos.1;
            }
        }

        // rotate directions
        let nd = directions.pop_front().unwrap();
        directions.push_back(nd);
    }
    dbg!(num);
}
