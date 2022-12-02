use clap::{arg, command, value_parser};

mod day1;
mod day2;
mod utils;

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -t --task <TASK> "Task number"
            )
            .required(true)
            .value_parser(value_parser!(u8)),
        )
        .get_matches();

    let task = matches.get_one::<u8>("task").unwrap();

    match task {
        1 => {
            day1::run_easy();
            day1::run_hard()
        }
        2 => {
            day2::run_easy();
            day2::run_hard()
        }
        _ => println!("{} not found", task),
    }
}
