use itertools::Itertools;
use num::integer::lcm;
use std::collections::VecDeque;

use crate::utils::read_input;

#[derive(Debug)]
struct MonkeyState {
    items: VecDeque<i64>,
    operation: (String, String),
    divisibility: i64,
    monkey_on_positive: usize,
    monkey_on_negative: usize,
    monkey_inspections: i64,
}

pub fn run_easy() {
    let input = read_input("inputs/day11.txt");

    let mut monkeys = input
        .split(|x| x.is_empty())
        .into_iter()
        .map(|x| {
            let mut items = VecDeque::new();
            let mut line_iterator = x.iter().skip(1);
            let items_line = line_iterator
                .next()
                .unwrap()
                .split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse::<i64>().unwrap());
            items.extend(items_line);

            let operation = line_iterator
                .next()
                .unwrap()
                .split("= old")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .collect_vec();

            let procedure = operation[0].to_string();
            let value = operation[1].to_string();

            let test_divisibility = line_iterator
                .next()
                .unwrap()
                .split("by")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();
            let monkey_on_positive = line_iterator
                .next()
                .unwrap()
                .split("monkey")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();
            let monkey_on_negative = line_iterator
                .next()
                .unwrap()
                .split("monkey")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();
            MonkeyState {
                items,
                operation: (procedure, value),
                divisibility: test_divisibility,
                monkey_on_positive: monkey_on_positive as usize,
                monkey_on_negative: monkey_on_negative as usize,
                monkey_inspections: 0,
            }
        })
        .collect_vec();

    for i in 0..20 {
        for m in 0..(monkeys.len()) {
            while !monkeys[m].items.is_empty() {
                let mut item = monkeys[m].items.pop_front().unwrap();

                let value = if monkeys[m].operation.1 == "old" {
                    item
                } else {
                    monkeys[m].operation.1.parse::<i64>().unwrap()
                };

                match monkeys[m].operation.0.as_ref() {
                    "+" => {
                        item += value;
                    }
                    "*" => {
                        item *= value;
                    }
                    _ => unreachable!(),
                }

                monkeys[m].monkey_inspections += 1;

                item /= 3;

                let idx = if (item % monkeys[m].divisibility) == 0 {
                    monkeys[m].monkey_on_positive
                } else {
                    monkeys[m].monkey_on_negative
                };
                monkeys[idx].items.push_back(item);
            }
        }
    }

    let m = monkeys
        .into_iter()
        .sorted_by_key(|x| x.monkey_inspections)
        .rev()
        .take(2)
        .map(|x| x.monkey_inspections)
        .collect_vec();

    println!("{}", m[0] * m[1]);
}

pub fn run_hard() {
    let input = read_input("inputs/day11.txt");

    let mut monkeys = input
        .split(|x| x.is_empty())
        .into_iter()
        .map(|x| {
            let mut items = VecDeque::new();
            let mut line_iterator = x.iter().skip(1);
            let items_line = line_iterator
                .next()
                .unwrap()
                .split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse::<i64>().unwrap());
            items.extend(items_line);

            let operation = line_iterator
                .next()
                .unwrap()
                .split("= old")
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .collect_vec();

            let procedure = operation[0].to_string();
            let value = operation[1].to_string();

            let test_divisibility = line_iterator
                .next()
                .unwrap()
                .split("by")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();
            let monkey_on_positive = line_iterator
                .next()
                .unwrap()
                .split("monkey")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();
            let monkey_on_negative = line_iterator
                .next()
                .unwrap()
                .split("monkey")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i64>()
                .unwrap();
            MonkeyState {
                items,
                operation: (procedure, value),
                divisibility: test_divisibility,
                monkey_on_positive: monkey_on_positive as usize,
                monkey_on_negative: monkey_on_negative as usize,
                monkey_inspections: 0,
            }
        })
        .collect_vec();

    let divisibilities = monkeys.iter().map(|m| m.divisibility).collect_vec();
    let mut divisibility_lcm = 1;

    for i in divisibilities {
        divisibility_lcm = lcm(divisibility_lcm, i);
    }

    for i in 0..10000 {
        for m in 0..(monkeys.len()) {
            while !monkeys[m].items.is_empty() {
                let mut item = monkeys[m].items.pop_front().unwrap();

                let value = if monkeys[m].operation.1 == "old" {
                    item
                } else {
                    monkeys[m].operation.1.parse::<i64>().unwrap()
                };

                match monkeys[m].operation.0.as_ref() {
                    "+" => {
                        item += value;
                    }
                    "*" => {
                        item *= value;
                    }
                    _ => unreachable!(),
                }

                item %= divisibility_lcm;

                monkeys[m].monkey_inspections += 1;

                let idx = if (item % monkeys[m].divisibility) == 0 {
                    monkeys[m].monkey_on_positive
                } else {
                    monkeys[m].monkey_on_negative
                };
                monkeys[idx].items.push_back(item);
            }
        }
    }

    let m = monkeys
        .into_iter()
        .sorted_by_key(|x| x.monkey_inspections)
        .rev()
        .take(2)
        .map(|x| x.monkey_inspections)
        .collect_vec();

    println!("{}", m[0] * m[1]);
}
