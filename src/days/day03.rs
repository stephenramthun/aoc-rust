use std::fs;
use regex::Regex;

pub fn solver() {
    println!("Day 3:");

    let input =
        fs::read_to_string("./src/days/day03.txt").expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}


fn solve_part_1(input: &str) -> usize {
    let mult_regex = Regex::new(r#"mul(\(\d+,\d+\))"#).unwrap();
    let mult_operations = mult_regex.find_iter(input);

    mult_operations.map(|m| mult(m.as_str())).sum()
}

fn solve_part_2(input: &str) -> usize {
    let ops_regex = Regex::new(r#"mul(\(\d+,\d+\))|(don't\(\)|(do\(\)))"#).unwrap();
    let ops = ops_regex.find_iter(&input);

    let mut enabled = true;
    let mut sum = 0;
    for op in ops {
        let name: &str = op.as_str().split("(").collect::<Vec<&str>>()[0];

        match name {
            "mul" => {
                if enabled {
                    sum += mult(op.as_str())
                }
            },
            "don't" => { enabled = false },
            "do" => { enabled = true },
            _ => panic!("Encountered unknown operation: {name}"),
        }
    }

    sum
}

fn mult(input: &str) -> usize {
    let digits_regex = Regex::new(r#"\d+"#).unwrap();
    let digits: Vec<usize> = digits_regex
        .find_iter(input)
        .map(|x| x.as_str().parse::<usize>().unwrap())
        .collect();
    digits[0] * digits[1]
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::days::day03::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/days/day03.txt").unwrap();
        let solution = solve_part_1(&input);
        assert_eq!(solution, 175700056);
    }

    #[test]
    fn test_input_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let solution = solve_part_1(input);
        assert_eq!(solution, 161);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/days/day03.txt").unwrap();
        let solution = solve_part_2(&input);
        assert_eq!(solution, 71668682);
    }

    #[test]
    fn test_input_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let solution = solve_part_2(input);
        assert_eq!(solution, 48);
    }
}
