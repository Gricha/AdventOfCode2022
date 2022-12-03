use crate::utils::read_input;

pub fn run_easy() {
    let input = read_input("inputs/day1.txt");

    let segments = input.split(|x| x.is_empty());

    let mut max = 0;

    for seg in segments {
        let value: i32 = seg.iter().map(|x| x.parse::<i32>().unwrap()).sum();
        max = std::cmp::max(value, max);
    }

    println!("{}", max);
}

pub fn run_hard() {
    let input = read_input("inputs/day1.txt");

    let segments = input.split(|x| x.is_empty());

    let mut data: Vec<i32> = vec![];

    for seg in segments {
        data.push(seg.iter().map(|x| x.parse::<i32>().unwrap()).sum());
    }

    data.sort();
    let value: i32 = data.into_iter().rev().take(3).sum();

    println!("{}", value);
}
