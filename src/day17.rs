use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_input;

fn rock_shapes() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)],
        vec![(2, 0), (2, 1), (2, 2), (1, 0), (0, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ]
}

fn leftmost_edges() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(0, 0)],
        vec![(1, 0), (0, 1), (1, 2)],
        vec![(2, 2), (2, 1), (0, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1)],
    ]
}

fn rightmost_edges() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(3, 0)],
        vec![(1, 0), (1, 2), (2, 1)],
        vec![(2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(1, 0), (1, 1)],
    ]
}

fn downmost_edges() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (2, 1)],
        vec![(2, 0), (1, 0), (0, 0)],
        vec![(0, 0)],
        vec![(0, 0), (1, 0)],
    ]
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

pub fn run_easy() {
    let input = read_input("inputs/day17.txt").into_iter().next().unwrap();

    let turns = input
        .chars()
        .map(|x| match x {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut max_height = 0;

    let mut cave_state: Vec<[u8; 7]> = vec![];

    let mut current_rock_idx = 0;
    let mut current_rock = rock_shapes()[0].clone();
    let mut current_position = (2, 3);

    let mut current_left_edges = leftmost_edges()[0].clone();
    let mut current_right_edges = rightmost_edges()[0].clone();
    let mut current_down_edges = downmost_edges()[0].clone();
    let mut stopped_rocks = 0;

    let mut i = 0;

    loop {
        let turn = turns[i % turns.len()];
        // first simulate jet move
        match turn {
            Direction::Left => {
                // if any edge is blocked, piece is blocked
                let mut is_blocked = false;

                for edge in current_left_edges.iter() {
                    let edge_map_position =
                        (current_position.0 + edge.0, current_position.1 + edge.1);

                    if edge_map_position.0 == 0 {
                        is_blocked = true;
                        break;
                    }

                    if cave_state.len() > edge_map_position.1
                        && cave_state[edge_map_position.1][edge_map_position.0 - 1] == 1
                    {
                        is_blocked = true;
                        break;
                    }
                }

                if !is_blocked {
                    current_position = (current_position.0 - 1, current_position.1);
                }
            }
            Direction::Right => {
                let mut is_blocked = false;

                for edge in current_right_edges.iter() {
                    let edge_map_position =
                        (current_position.0 + edge.0, current_position.1 + edge.1);

                    if edge_map_position.0 == 6 {
                        is_blocked = true;
                        break;
                    }

                    if cave_state.len() > edge_map_position.1
                        && cave_state[edge_map_position.1][edge_map_position.0 + 1] == 1
                    {
                        is_blocked = true;
                        break;
                    }
                }
                if !is_blocked {
                    current_position = (current_position.0 + 1, current_position.1);
                }
            }
        }
        // then see if i can move down, if i can cool, if i can't stop the rock, rotate piece
        let mut is_blocked = false;

        for edge in current_down_edges.iter() {
            let edge_map_position = (current_position.0 + edge.0, current_position.1 + edge.1);

            if edge_map_position.1 == 0 {
                is_blocked = true;
                break;
            }

            if cave_state.len() >= edge_map_position.1
                && cave_state[edge_map_position.1 - 1][edge_map_position.0] == 1
            {
                is_blocked = true;
                break;
            }
        }

        if !is_blocked {
            current_position = (current_position.0, current_position.1 - 1);
        } else {
            for edge in current_rock.iter() {
                let edge_position = (current_position.0 + edge.0, current_position.1 + edge.1);
                while cave_state.len() <= edge_position.1 {
                    cave_state.push([0; 7]);
                }

                cave_state[edge_position.1][edge_position.0] = 1;
                max_height = std::cmp::max(max_height, edge_position.1);
            }
            stopped_rocks += 1;
            if stopped_rocks == 2022 {
                break;
            }

            current_rock_idx = (current_rock_idx + 1) % 5;
            current_rock = rock_shapes()[current_rock_idx].clone();
            current_position = (2, max_height + 4);

            current_left_edges = leftmost_edges()[current_rock_idx].clone();
            current_right_edges = rightmost_edges()[current_rock_idx].clone();
            current_down_edges = downmost_edges()[current_rock_idx].clone();
        }

        i += 1;
    }

    println!("{}", max_height + 1);
}

pub fn run_hard() {
    let input = read_input("inputs/day17.txt").into_iter().next().unwrap();

    let turns = input
        .chars()
        .map(|x| match x {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut max_height = 0;

    let mut cave_state: Vec<[u8; 7]> = vec![];

    let mut current_rock_idx = 0;
    let mut current_rock = rock_shapes()[0].clone();
    let mut current_position = (2, 3);

    let mut current_left_edges = leftmost_edges()[0].clone();
    let mut current_right_edges = rightmost_edges()[0].clone();
    let mut current_down_edges = downmost_edges()[0].clone();
    let mut stopped_rocks = 0;

    let mut i = 0;

    let mut max_height_cache = vec![0];

    let mut tower_cache = HashMap::<(usize, String, usize), usize>::new();
    let mut search_cycle = true;

    loop {
        let turn = turns[i % turns.len()];

        let turn_idx = i % turns.len();
        // first simulate jet move
        match turn {
            Direction::Left => {
                // if any edge is blocked, piece is blocked
                let mut is_blocked = false;

                for edge in current_left_edges.iter() {
                    let edge_map_position =
                        (current_position.0 + edge.0, current_position.1 + edge.1);

                    if edge_map_position.0 == 0 {
                        is_blocked = true;
                        break;
                    }

                    if cave_state.len() > edge_map_position.1
                        && cave_state[edge_map_position.1][edge_map_position.0 - 1] == 1
                    {
                        is_blocked = true;
                        break;
                    }
                }

                if !is_blocked {
                    current_position = (current_position.0 - 1, current_position.1);
                }
            }
            Direction::Right => {
                let mut is_blocked = false;

                for edge in current_right_edges.iter() {
                    let edge_map_position =
                        (current_position.0 + edge.0, current_position.1 + edge.1);

                    if edge_map_position.0 == 6 {
                        is_blocked = true;
                        break;
                    }

                    if cave_state.len() > edge_map_position.1
                        && cave_state[edge_map_position.1][edge_map_position.0 + 1] == 1
                    {
                        is_blocked = true;
                        break;
                    }
                }
                if !is_blocked {
                    current_position = (current_position.0 + 1, current_position.1);
                }
            }
        }
        // then see if i can move down, if i can cool, if i can't stop the rock, rotate piece
        let mut is_blocked = false;

        for edge in current_down_edges.iter() {
            let edge_map_position = (current_position.0 + edge.0, current_position.1 + edge.1);

            if edge_map_position.1 == 0 {
                is_blocked = true;
                break;
            }

            if cave_state.len() >= edge_map_position.1
                && cave_state[edge_map_position.1 - 1][edge_map_position.0] == 1
            {
                is_blocked = true;
                break;
            }
        }

        if !is_blocked {
            current_position = (current_position.0, current_position.1 - 1);
        } else {
            let old_max_height = max_height;

            for edge in current_rock.iter() {
                let edge_position = (current_position.0 + edge.0, current_position.1 + edge.1);
                while cave_state.len() <= edge_position.1 {
                    cave_state.push([0; 7]);
                }

                cave_state[edge_position.1][edge_position.0] = 1;

                max_height = std::cmp::max(max_height, edge_position.1);
            }

            stopped_rocks += 1;

            max_height_cache.push(max_height - old_max_height);

            current_rock_idx = (current_rock_idx + 1) % 5;
            current_rock = rock_shapes()[current_rock_idx].clone();
            current_position = (2, max_height + 4);

            current_left_edges = leftmost_edges()[current_rock_idx].clone();
            current_right_edges = rightmost_edges()[current_rock_idx].clone();
            current_down_edges = downmost_edges()[current_rock_idx].clone();

            if stopped_rocks > 10000 {
                let tower_top_stringified = cave_state
                    .iter()
                    .rev()
                    .take(50)
                    .map(|x| x.iter().map(|c| if *c == 0 { "0" } else { "1" }).join(""))
                    .join("");
                if let Some(value) =
                    tower_cache.get(&(turn_idx, tower_top_stringified.clone(), current_rock_idx))
                {
                    println!("current height: {}", max_height);
                    let cycle_length = stopped_rocks - value;
                    println!("Cycle length: {}", cycle_length);
                    let rounds_to_perform = (1000000000000 - stopped_rocks);
                    println!("Remaining rounds: {}", rounds_to_perform);
                    let cycles_to_perform = (rounds_to_perform / cycle_length);
                    println!("Remaining cyles: {}", cycles_to_perform);
                    let height_gained_per_cycle: usize = max_height_cache
                        [(stopped_rocks - cycle_length)..stopped_rocks]
                        .iter()
                        .sum();
                    println!("Height gained per cycle: {}", height_gained_per_cycle);
                    let added_height = cycles_to_perform * height_gained_per_cycle;
                    println!("Total height gained thru missed cycles: {}", added_height);

                    let missing_rounds = rounds_to_perform - (cycles_to_perform * cycle_length);
                    let missing_height: usize = max_height_cache[(stopped_rocks - cycle_length)
                        ..=(stopped_rocks - cycle_length + missing_rounds)]
                        .iter()
                        .sum();

                    println!("{}", max_height + added_height + missing_height);
                    return;
                } else {
                    tower_cache.insert(
                        (turn_idx, tower_top_stringified, current_rock_idx),
                        stopped_rocks,
                    );
                }
            }
        }

        i += 1;
    }
}
