pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub fn get_solver(day: u8) -> fn() {
    match day {
        1 => day01::solver,
        2 => day02::solver,
        3 => day03::solver,
        4 => day04::solver,
        5 => day05::solver,
        6 => day06::solver,
        7 => day07::solver,
        8 => day08::solver,
        9 => day09::solver,
        10 => day10::solver,
        11 => day11::solver,
        _ => panic!("Unknown day: {}", day),
    }
}