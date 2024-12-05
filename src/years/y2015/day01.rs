use std::fs;

pub fn solver() {
    println!("Day 1:");

    let input = fs::read_to_string("./src/years/y2015/day01.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> isize {
    input.chars().fold(0, |floor, stair| floor + if stair == '(' { 1 } else { -1 })
}

fn solve_part_2(input: &str) -> usize {
    let mut floor = 0;
    for (i, char) in input.chars().enumerate() {
        floor += if char == '(' { 1 } else { -1 };
        if floor < 0 {
            return i
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::years::y2015::day01::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2015/day01.txt").unwrap();
        assert_eq!(solve_part_1(&input), 280);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2015/day01.txt").unwrap();
        assert_eq!(solve_part_2(&input), 1797);
    }
}
