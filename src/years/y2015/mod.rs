pub mod day01;

pub fn get_solver(day: u8) -> fn() {
    match day {
        1 => day01::solver,
        _ => panic!("Unknown day: {}", day),
    }
}