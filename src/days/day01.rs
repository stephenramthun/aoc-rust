use std::fs;
use std::iter::zip;

pub fn solver() {
    println!("Day 1:");

    let input = fs::read_to_string("./src/days/day01.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    let (mut list_a, mut list_b): (Vec<usize>, Vec<usize>) = get_lists(input);

    list_a.sort();
    list_b.sort();

    zip(list_a.clone(), list_b.clone())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>()
}

fn solve_part_2(input: &str) -> usize {
    let (list_a, list_b): (Vec<usize>, Vec<usize>) = get_lists(input);

    list_a.iter()
        .map(|&a| a * list_b.iter().filter(|&&b| b == a).count())
        .sum::<usize>()
}

fn get_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.trim()
        .split("\n")
        .map(|x| {
            let (a, b) = x.split_once(char::is_whitespace).unwrap();
            (a.parse::<usize>().unwrap(), b.trim().parse::<usize>().unwrap())
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_part_1() {
        let contents = fs::read_to_string("./src/days/day01.txt")
            .expect("Should have been able to read the file");
        assert_eq!(solve_part_1(&contents), 3246517)
    }

    #[test]
    fn test_input_part_1() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#;
        assert_eq!(solve_part_1(input), 11)
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#;
        assert_eq!(solve_part_2(input), 31)
    }
}
