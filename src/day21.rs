use std::collections::HashMap;

use crate::utils::read_input;

#[derive(Debug)]
enum Node {
    Number(i64),
    Operation(String, char, String),
}

fn find_value(values: &HashMap<String, Node>, value: &str) -> i64 {
    let node = values.get(value).unwrap();

    match node {
        Node::Number(v) => *v,
        Node::Operation(id1, op, id2) => {
            let v1 = find_value(values, id1);
            let v2 = find_value(values, id2);

            match op {
                '+' => v1 + v2,
                '-' => v1 - v2,
                '*' => v1 * v2,
                '/' => v1 / v2,
                _ => unreachable!(),
            }
        }
    }
}

pub fn run_easy() {
    let input = read_input("inputs/day21.txt")
        .into_iter()
        .map(|x| {
            let mut vals = x.split(": ").into_iter();
            let id = vals.next().unwrap();
            let res = vals.next().unwrap();
            let outcome = match res.parse::<i64>() {
                Ok(val) => Node::Number(val),
                Err(_) => {
                    let mut s = res.split_ascii_whitespace();
                    Node::Operation(
                        s.next().unwrap().to_string(),
                        s.next().unwrap().chars().next().unwrap().clone(),
                        s.next().unwrap().to_string(),
                    )
                }
            };
            (id.to_string(), outcome)
        })
        .collect::<HashMap<String, Node>>();

    dbg!(find_value(&input, "root"));
}

fn maybe_find_value(values: &HashMap<String, Node>, value: &str) -> Option<i64> {
    let node = values.get(value).unwrap();

    if value == "humn" {
        return None;
    }

    Some(match node {
        Node::Number(v) => *v,
        Node::Operation(id1, op, id2) => {
            let maybe_v1 = maybe_find_value(values, id1);
            let maybe_v2 = maybe_find_value(values, id2);

            if maybe_v1.is_none() || maybe_v2.is_none() {
                return None;
            }

            let v1 = maybe_v1.unwrap();
            let v2 = maybe_v2.unwrap();

            match op {
                '+' => v1 + v2,
                '-' => v1 - v2,
                '*' => v1 * v2,
                '/' => v1 / v2,
                _ => unreachable!(),
            }
        }
    })
}

fn solve_to_value(values: &HashMap<String, Node>, value: i64, id: &str) -> i64 {
    let node = values.get(id).unwrap();

    match node {
        Node::Number(_x) => {
            assert!(id == "humn");
            value
        }
        Node::Operation(id1, op, id2) => {
            let mut val1 = maybe_find_value(values, id1);
            let mut val2 = maybe_find_value(values, id2);
            let mut operation_id = id2;
            let mut negate = true;

            assert!((val1.is_none() && val2.is_some()) || (val2.is_none() && val1.is_some()));

            if val1.is_none() {
                val1 = val2;
                val2 = None;
                operation_id = id1;
                negate = false;
            }

            let mut new_value = val1.unwrap();

            match op {
                '+' => {
                    new_value = value - new_value;
                }
                '-' => {
                    if negate {
                        new_value -= value;
                    } else {
                        new_value += value;
                    }
                }
                '*' => {
                    new_value = value / new_value;
                }
                '/' => {
                    if negate {
                        new_value /= value;
                    } else {
                        new_value *= value;
                    }
                }
                _ => unreachable!(),
            }

            solve_to_value(values, new_value, operation_id)
        }
    }
}

pub fn run_hard() {
    let input = read_input("inputs/day21.txt")
        .into_iter()
        .map(|x| {
            let mut vals = x.split(": ").into_iter();
            let id = vals.next().unwrap();
            let res = vals.next().unwrap();
            let outcome = match res.parse::<i64>() {
                Ok(val) => Node::Number(val),
                Err(_) => {
                    let mut s = res.split_ascii_whitespace();
                    Node::Operation(
                        s.next().unwrap().to_string(),
                        s.next().unwrap().chars().next().unwrap().clone(),
                        s.next().unwrap().to_string(),
                    )
                }
            };
            (id.to_string(), outcome)
        })
        .collect::<HashMap<String, Node>>();

    let (id1, id2) = match input.get("root").unwrap() {
        Node::Number(_) => unreachable!(),
        Node::Operation(v1, _, v2) => (v1, v2),
    };

    let value1 = maybe_find_value(&input, id1);
    let value2 = maybe_find_value(&input, id2);

    if value1.is_none() {
        dbg!(solve_to_value(&input, value2.unwrap(), id1));
    } else {
        dbg!(solve_to_value(&input, value1.unwrap(), id2));
    }
}
