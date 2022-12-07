use std::collections::BTreeMap;

use itertools::Itertools;

use crate::utils::read_input;

#[derive(Debug)]
enum Command {
    Ls,
    GoUp,
    GoInside(String),
}

#[derive(Debug)]
enum LineOutput {
    Leaf(u32, String),
    Dir(String),
    Cmd(Command),
}

fn parse_line(line: &str) -> LineOutput {
    match line {
        "$ ls" => LineOutput::Cmd(Command::Ls),
        "$ cd .." => LineOutput::Cmd(Command::GoUp),
        l if l.starts_with("$ cd") => {
            LineOutput::Cmd(Command::GoInside(l.split(" ").last().unwrap().to_owned()))
        }
        l if l.starts_with("dir ") => LineOutput::Dir(l.split(' ').last().unwrap().to_owned()),
        l => {
            let (first, sec) = l
                .split_ascii_whitespace()
                .into_iter()
                .collect_tuple()
                .unwrap();

            LineOutput::Leaf(first.parse::<u32>().unwrap(), sec.to_owned())
        }
    }
}

fn calc_size(
    path: String,
    children: &BTreeMap<String, Vec<String>>,
    sizes: &BTreeMap<String, u32>,
) -> u32 {
    if sizes.contains_key(&path) {
        return *sizes.get(&path).unwrap();
    } else {
        let mut size = 0;
        for child in children.get(&path).unwrap() {
            size += calc_size(format!("{}/{}", path.to_owned(), child), children, sizes);
        }
        size
    }
}

pub fn run_easy() {
    let commands = read_input("inputs/day7.txt")
        .into_iter()
        // first is not mattering
        .skip(1)
        .map(|x| parse_line(&x))
        .collect_vec();

    let mut children = BTreeMap::new();
    let mut sizes = BTreeMap::new();
    let mut directories = vec![];

    let mut current_location = "".to_string();

    for c in commands.into_iter() {
        match c {
            LineOutput::Leaf(size, name) => {
                if !children.contains_key(&current_location) {
                    children.insert(current_location.to_owned(), vec![]);
                }
                children
                    .get_mut(&current_location)
                    .unwrap()
                    .push(name.to_owned());
                sizes.insert(format!("{}/{}", current_location, name), size);
            }
            LineOutput::Dir(name) => {
                if !children.contains_key(&current_location) {
                    children.insert(current_location.to_owned(), vec![]);
                }
                children
                    .get_mut(&current_location)
                    .unwrap()
                    .push(name.to_string());
                directories.push(format!("{}/{}", current_location, name))
            }
            LineOutput::Cmd(command) => match command {
                Command::GoUp => {
                    current_location = current_location
                        .split('/')
                        .rev()
                        .skip(1)
                        .collect_vec()
                        .into_iter()
                        .rev()
                        .join("/");
                }
                Command::GoInside(dir) => {
                    current_location = format!("{}/{}", &current_location, dir);
                }
                Command::Ls => (),
            },
        }
    }

    let mut sum = 0;
    for i in directories {
        let size = calc_size(i, &children, &sizes);
        if size <= 100000 {
            sum += size;
        }
    }
    println!("{}", sum);
}

pub fn run_hard() {
    let commands = read_input("inputs/day7.txt")
        .into_iter()
        // first is not mattering
        .skip(1)
        .map(|x| parse_line(&x))
        .collect_vec();

    let mut children = BTreeMap::new();
    let mut sizes = BTreeMap::new();
    let mut directories = vec![];

    let mut current_location = "".to_string();

    for c in commands.into_iter() {
        match c {
            LineOutput::Leaf(size, name) => {
                if !children.contains_key(&current_location) {
                    children.insert(current_location.to_owned(), vec![]);
                }
                children
                    .get_mut(&current_location)
                    .unwrap()
                    .push(name.to_owned());
                sizes.insert(format!("{}/{}", current_location, name), size);
            }
            LineOutput::Dir(name) => {
                if !children.contains_key(&current_location) {
                    children.insert(current_location.to_owned(), vec![]);
                }
                children
                    .get_mut(&current_location)
                    .unwrap()
                    .push(name.to_string());
                directories.push(format!("{}/{}", current_location, name))
            }
            LineOutput::Cmd(command) => match command {
                Command::GoUp => {
                    current_location = current_location
                        .split('/')
                        .rev()
                        .skip(1)
                        .collect_vec()
                        .into_iter()
                        .rev()
                        .join("/");
                }
                Command::GoInside(dir) => {
                    current_location = format!("{}/{}", &current_location, dir);
                }
                Command::Ls => (),
            },
        }
    }

    let top_level_space_used = calc_size("".to_string(), &children, &sizes);

    let min_size_required = top_level_space_used - 40000000;

    let mut smallest_enough = u32::MAX;

    for i in directories {
        let size = calc_size(i, &children, &sizes);
        if size >= min_size_required && size < smallest_enough {
            smallest_enough = size;
        }
    }

    println!("{}", smallest_enough);
}
