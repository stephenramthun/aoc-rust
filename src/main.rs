use std::env;
use crate::days::day01;

mod days;

fn main() {
    let day = get_day();

    let solver = match day {
        1 => day01::solver,
        _ => panic!("Unknown day: {}", day),
    };

    solver();
}

fn get_day() -> u8 {
    let args: Vec<String> = env::args().collect();

    let day = args.iter()
        .find(|arg| { arg.contains("day=") })
        .expect("Please provide a day argument")
        .split("=")
        .last()
        .expect("Missing value part of day argument, expected format is \"day=\\{day\\}\"")
        .parse::<u8>()
        .expect("Invalid day argument, expected number");

    match day {
        1..=50 => day,
        _ => panic!("Day must be between 1 and 50 (inclusive)")
    }
}
