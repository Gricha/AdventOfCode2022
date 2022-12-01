use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;

pub fn read_numerical_input(path: &str) -> Vec<i32> {
    let path = Path::new(path);
    let file = BufReader::new(File::open(&path).unwrap());
    let mut v = Vec::new();
    for line in file.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        v.push(num);
    }
    v
}

pub fn read_input(path: &str) -> Vec<String> {
    let path = Path::new(path);
    let file = BufReader::new(File::open(&path).unwrap());
    let mut v = Vec::new();
    for l in file.lines() {
        v.push(l.unwrap());
    }
    v
}
