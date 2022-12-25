use crate::utils::read_input;

fn snafu_to_dec(snafu: &str) -> i64 {
    let powers = snafu.len();
    let mut num = 0;

    for (i, c) in snafu.chars().rev().enumerate() {
        match c {
            '0' => {}
            '1' => {
                num += 5_i64.pow(i.try_into().unwrap());
            }
            '2' => {
                num += 5_i64.pow(i.try_into().unwrap()) * 2;
            }
            '-' => num -= 5_i64.pow(i.try_into().unwrap()),
            '=' => num -= 5_i64.pow(i.try_into().unwrap()) * 2,
            _ => unreachable!(),
        }
    }

    num
}

fn dec_to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "".to_string();
    }

    match n % 5 {
        0 => dec_to_snafu(n / 5) + "0",
        1 => dec_to_snafu(n / 5) + "1",
        2 => dec_to_snafu(n / 5) + "2",
        3 => dec_to_snafu((n + 2) / 5) + "=",
        4 => dec_to_snafu((n + 1) / 5) + "-",
        _ => unreachable!(),
    }
}

pub fn run_easy() {
    let input: i64 = read_input("inputs/day25.txt")
        .into_iter()
        .map(|x| snafu_to_dec(&x))
        .map(|x| {
            dbg!(x);
            x
        })
        .sum();

    dbg!(dec_to_snafu(input));
}

pub fn run_hard() {}
