use regex::{Error, Regex};
use std::str::FromStr;

fn part1(input: &str) -> isize {
    parse_input(input).into_iter().map(|m| solve(m, 0)).sum()
}

fn part2(input: &str) -> isize {
    parse_input(input)
        .into_iter()
        .map(|m| solve(m, 10_000_000_000_000))
        .sum()
}

fn solve(machine: Machine, offset: isize) -> isize {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let px = machine.prize_location.0 + offset;
    let py = machine.prize_location.1 + offset;

    let d = by * ax - bx * ay;
    let da = by * px - bx * py;
    let db = ax * py - ay * px;

    if d == 0 {
        0
    } else if da % d == 0 && db % d == 0 {
        3 * (da / d) + db / d
    } else {
        0
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|chunk| Machine::from_str(chunk).unwrap())
        .collect()
}

type Point = (isize, isize);

#[derive(Debug)]
struct Machine {
    button_a: Point,
    button_b: Point,
    prize_location: Point,
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines().collect::<Vec<_>>();
        let (_, a) = lines[0].split_once(": ").unwrap();
        let (_, b) = lines[1].split_once(": ").unwrap();
        let (_, prize) = lines[2].split_once(": ").unwrap();

        Ok(Machine {
            button_a: parse_point(a),
            button_b: parse_point(b),
            prize_location: parse_point(prize),
        })
    }
}

fn parse_point(input: &str) -> Point {
    let regex = Regex::new(r"\d+").unwrap();
    match regex
        .find_iter(&input)
        .map(|m| m.as_str().parse::<isize>().unwrap())
        .collect::<Vec<_>>()[..]
    {
        [a, b] => (a, b),
        _ => panic!("Missing values for X and Y"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_case_1() {
        let input = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"#
            .trim();
        assert_eq!(part1(&input), 280);
    }

    #[test]
    fn test_case_2() {
        let input = r#"
Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"#
            .trim();
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_case_3() {
        let input = r#"
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450"#
            .trim();
        assert_eq!(part1(&input), 200);
    }

    #[test]
    fn test_case_4() {
        let input = r#"
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#
            .trim();
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("./src/years/y2024/day13.txt").unwrap();
        assert_eq!(part1(&input), 29436);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("./src/years/y2024/day13.txt").unwrap();
        assert_eq!(part2(&input), 103729094227877);
    }
}
