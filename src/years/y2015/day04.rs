use std::fs;
use md5;

pub fn solver() {
    println!("Day 4:");

    let input = fs::read_to_string("./src/years/y2015/day04.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    solve(input, "00000")
}

fn solve_part_2(input: &str) -> usize {
    solve(input, "000000")
}

fn solve(input: &str, prefix: &str) -> usize {
    let mut current = 1;
    loop {
        let hash = md5::compute(format!("{input}{current}"));
        if format!("{:x}", hash).starts_with(prefix) {
            return current;
        }
        current += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::years::y2015::day04::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2015/day04.txt").unwrap();
        assert_eq!(solve_part_1(&input), 254575);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2015/day04.txt").unwrap();
        assert_eq!(solve_part_2(&input), 1038736);
    }
}