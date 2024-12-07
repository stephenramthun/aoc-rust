use std::fs;

trait Operation {
    fn calc(&self, a: i64, b: i64) -> i64;
    fn should_continue(&self, a: i64, b: i64) -> bool;
}

struct Subtract;

impl Operation for Subtract {
    fn calc(&self, a: i64, b: i64) -> i64 {
        a - b
    }
    fn should_continue(&self, a: i64, b: i64) -> bool {
        a > b
    }
}

struct Divide;

impl Operation for Divide {
    fn calc(&self, a: i64, b: i64) -> i64 {
        a / b
    }
    fn should_continue(&self, a: i64, b: i64) -> bool {
        a % b == 0
    }
}

struct Concatenate;

impl Operation for Concatenate {
    fn calc(&self, a: i64, mut b: i64) -> i64 {
        let mut multiplier = 1;
        while b > 0 {
            b /= 10;
            multiplier *= 10;
        }

        a / multiplier
    }
    fn should_continue(&self, a: i64, b: i64) -> bool {
        let mut multiplier = 1;
        let mut c = b;
        while c > 0 {
            c /= 10;
            multiplier *= 10;
        }

        (a % multiplier) == b
    }
}

pub fn solver() {
    println!("Day 7:");

    let input = fs::read_to_string("./src/years/y2024/day07.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> i64 {
    let ops: [Box<dyn Operation>; 2] = [Box::new(Subtract), Box::new(Divide)];
    parse_input(input)
        .iter()
        .filter(|&(target, values)| is_solveable((target.clone(), values), &ops))
        .fold(0, |sum, (value, _)| sum + value)
}

fn solve_part_2(input: &str) -> i64 {
    let ops: [Box<dyn Operation>; 3] =
        [Box::new(Subtract), Box::new(Divide), Box::new(Concatenate)];
    parse_input(input)
        .iter()
        .filter(|&(target, values)| is_solveable((target.clone(), values), &ops))
        .fold(0, |sum, (value, _)| sum + value)
}

fn is_solveable(equation: (i64, &[i64]), ops: &[Box<dyn Operation>]) -> bool {
    let (target, values) = equation;

    if values.len() == 1 {
        return values[0] == target;
    }

    let last = values[values.len() - 1];
    let rest = &values[..values.len() - 1];

    ops.iter().any(|op| {
        op.should_continue(target, last) && is_solveable((op.calc(target, last), rest), ops)
    })
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (sum, values) = line.split_once(": ").unwrap();
            (
                sum.parse().unwrap(),
                values
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day07::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#
        .trim();
        assert_eq!(solve_part_1(input), 3749);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day07.txt").unwrap();
        assert_eq!(solve_part_1(&input), 1582598718861);
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#
        .trim();
        assert_eq!(solve_part_2(input), 11387);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day07.txt").unwrap();
        assert_eq!(solve_part_2(&input), 165278151522644);
    }
}
