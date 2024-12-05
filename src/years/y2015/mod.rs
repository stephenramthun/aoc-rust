pub mod day01;
pub mod day02;
pub mod day03;

pub fn get_solver(day: u8) -> fn() {
    match day {
        1 => day01::solver,
        2 => day02::solver,
        3 => day03::solver,
        _ => panic!("Unknown day: {}", day),
    }
}