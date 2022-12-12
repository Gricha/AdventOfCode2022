use std::{cmp::Ordering, collections::BinaryHeap};

use crate::utils::read_input;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run_easy() {
    let input = read_input("inputs/day12.txt");

    let cols = input[0].len();
    let rows = input.len();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = [[-1; 200]; 200];

    for (j, row) in input.iter().enumerate() {
        for (i, col) in row.chars().enumerate() {
            if col == 'S' {
                start = (i, j);
                map[i][j] = (b'a') as i32;
            } else if col == 'E' {
                end = (i, j);
                map[i][j] = (b'z') as i32;
            } else {
                map[i][j] = col as i32;
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut dist = [[usize::MAX; 200]; 200];

    dist[start.0 as usize][start.1 as usize] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            println!("{}", cost);
            return;
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        let mut edges = vec![];

        if position.0 > 0 && ((map[position.0 - 1][position.1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0 - 1, position.1))
        }
        if position.0 < cols - 1
            && ((map[position.0 + 1][position.1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0 + 1, position.1))
        }
        if position.1 > 0 && ((map[position.0][position.1 - 1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0, position.1 - 1))
        }
        if position.1 < rows - 1
            && ((map[position.0][position.1 + 1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0, position.1 + 1))
        }

        for edge in edges {
            let next = State {
                cost: cost + 1,
                position: edge,
            };

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }
}

pub fn run_hard() {
    let input = read_input("inputs/day12.txt");

    let cols = input[0].len();
    let rows = input.len();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = [[-1; 200]; 200];
    let mut heap = BinaryHeap::new();
    let mut dist = [[usize::MAX; 200]; 200];

    for (j, row) in input.iter().enumerate() {
        for (i, col) in row.chars().enumerate() {
            if col == 'S' {
                start = (i, j);
                map[i][j] = (b'a') as i32;
            } else if col == 'E' {
                end = (i, j);
                map[i][j] = (b'z') as i32;
            } else {
                if col == 'a' {
                    dist[i][j] = 0;
                    heap.push(State {
                        cost: 0,
                        position: (i, j),
                    })
                }
                map[i][j] = col as i32;
            }
        }
    }

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            println!("{}", cost);
            return;
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        let mut edges = vec![];

        if position.0 > 0 && ((map[position.0 - 1][position.1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0 - 1, position.1))
        }
        if position.0 < cols - 1
            && ((map[position.0 + 1][position.1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0 + 1, position.1))
        }
        if position.1 > 0 && ((map[position.0][position.1 - 1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0, position.1 - 1))
        }
        if position.1 < rows - 1
            && ((map[position.0][position.1 + 1] - map[position.0][position.1]) <= 1)
        {
            edges.push((position.0, position.1 + 1))
        }

        for edge in edges {
            let next = State {
                cost: cost + 1,
                position: edge,
            };

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }
}
