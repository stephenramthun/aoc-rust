extern crate core;

use std::env;
use std::time::Instant;
use years::{y2015, y2024};

mod years;

fn main() {
    let day = get_day();
    let year = get_year();

    let solver = match year {
        2015 => y2015::get_solver(day),
        2024 => y2024::get_solver(day),
        _ => panic!("Unknown year: {}", year),
    };

    let now = Instant::now();
    solver();
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}

fn get_year() -> u16 {
    let args: Vec<String> = env::args().collect();

    let year = args
        .iter()
        .find(|arg| arg.contains("year="))
        .expect("Please provide a year argument")
        .split("=")
        .last()
        .expect("Missing value part of year argument, expected format is \"year=\\{year\\}\"")
        .parse::<u16>()
        .expect("Invalid day argument, expected number");

    match year {
        2015..=2024 => year,
        _ => panic!("Year must be between 2015 and 2024 (inclusive)"),
    }
}

fn get_day() -> u8 {
    let args: Vec<String> = env::args().collect();

    let day = args
        .iter()
        .find(|arg| arg.contains("day="))
        .expect("Please provide a day argument")
        .split("=")
        .last()
        .expect("Missing value part of day argument, expected format is \"day=\\{day\\}\"")
        .parse::<u8>()
        .expect("Invalid day argument, expected number");

    match day {
        1..=50 => day,
        _ => panic!("Day must be between 1 and 50 (inclusive)"),
    }
}
