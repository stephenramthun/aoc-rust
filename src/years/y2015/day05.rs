use fancy_regex::Regex;
use std::fs;

type Validator<'a> = fn(&'a str) -> bool;

pub fn solver() {
    println!("Day 5:");

    let input = fs::read_to_string("./src/years/y2015/day05.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    let validators: [Validator; 3] = [
        |x| contains_vowels(x, 3),
        |x| contains_duplicates(x),
        |x| does_not_contain(x, &["ab", "cd", "pq", "xy"]),
    ];
    input
        .lines()
        .filter(|line| is_nice(line, &validators))
        .count()
}

fn solve_part_2(input: &str) -> usize {
    let validators: [Validator; 2] = [|x| contains_pairs(x), |x| contains_sandwich(x)];
    input
        .lines()
        .filter(|line| is_nice(line, &validators))
        .count()
}

fn is_nice<'a>(string: &'a str, validators: &[Validator<'a>]) -> bool {
    validators.iter().all(|v| v(string))
}

fn contains_sandwich(string: &str) -> bool {
    let re = Regex::new(r"([a-z])[a-z](\1)").unwrap();
    re.is_match(string).unwrap()
}

fn contains_pairs(string: &str) -> bool {
    let re = Regex::new(r"([a-z][a-z])[a-z]*(\1)").unwrap();
    re.is_match(string).unwrap()
}

fn contains_vowels(string: &str, at_least: usize) -> bool {
    string.chars().filter(|&c| "aeiou".contains(c)).count() >= at_least
}

fn contains_duplicates(string: &str) -> bool {
    let re = Regex::new(r"([a-z])(\1)").unwrap();
    re.is_match(string).unwrap()
}

fn does_not_contain(string: &str, strings: &[&str]) -> bool {
    strings.iter().all(|&s| !string.contains(s))
}

#[cfg(test)]
mod tests {
    use crate::years::y2015::day05::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = fs::read_to_string("./src/years/y2015/day05.txt").unwrap();
        assert_eq!(solve_part_1(&input), 258);
    }

    #[test]
    fn test_input_part_2() {
        let input = fs::read_to_string("./src/years/y2015/day05.txt").unwrap();
        assert_eq!(solve_part_2(&input), 53);
    }
}
