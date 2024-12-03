use std::fs;

pub fn solver() {
    println!("Day 2:");

    let input =
        fs::read_to_string("./src/days/day02.txt").expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn is_ascending(report: &Vec<usize>) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

fn is_descending(report: &Vec<usize>) -> bool {
    report.windows(2).all(|w| w[0] > w[1])
}

fn is_tight(report: &Vec<usize>, max_gap: usize) -> bool {
    report.windows(2).all(|w| w[0].abs_diff(w[1]) <= max_gap)
}

fn is_valid(report: &&Vec<usize>) -> bool {
    (is_ascending(report) || is_descending(report)) && is_tight(report, 3)
}

fn solve_part_1(input: &str) -> usize {
    get_reports(input)
        .iter()
        .filter(is_valid)
        .count()
}

fn solve_part_2(input: &str) -> usize {
    get_reports(input)
        .iter()
        .filter(|&report| {
            for i in 0..report.len() {
                let mut copy = report.clone();
                copy.remove(i);

                if is_valid(&&copy) {
                    return true
                }
            }
            false
        })
        .count()
}

fn get_reports(input: &str) -> Vec<Vec<usize>> {
    input.trim().split("\n")
        .map(|level| {
            level.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::days::day02::{is_ascending, is_descending, is_tight, solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_is_ascending() {
        assert_eq!(is_ascending(&vec![1, 1]), false);
        assert_eq!(is_ascending(&vec![1, 2, 3, 2]), false);
        assert_eq!(is_ascending(&vec![3, 2, 1]), false);

        assert_eq!(is_ascending(&vec![1, 2]), true);
        assert_eq!(is_ascending(&vec![1, 2, 6, 19]), true);
    }

    #[test]
    fn test_is_descending() {
        assert_eq!(is_descending(&vec![1, 1]), false);
        assert_eq!(is_descending(&vec![1, 2, 3, 2]), false);

        assert_eq!(is_descending(&vec![2, 1]), true);
        assert_eq!(is_descending(&vec![10, 8, 6, 4]), true);
    }

    #[test]
    fn test_is_tight() {
        assert_eq!(is_tight(&vec![1, 2, 3], 2), true);
        assert_eq!(is_tight(&vec![1, 3, 5], 2), true);
        assert_eq!(is_tight(&vec![5, 3, 5], 2), true);
        assert_eq!(is_tight(&vec![1, 2, 5], 2), false);
        assert_eq!(is_tight(&vec![3, 2, 5], 2), false);
    }

    #[test]
    fn test_part_1() {
        let contents = fs::read_to_string("./src/days/day02.txt")
            .expect("Should have been able to read the file");
        assert_eq!(solve_part_1(&contents), 524)
    }

    #[test]
    fn test_input_part_1() {
        let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#;
        assert_eq!(solve_part_1(input), 2)
    }

    #[test]
    fn test_part_2() {
        let contents = fs::read_to_string("./src/days/day02.txt")
            .expect("Should have been able to read the file");
        assert_eq!(solve_part_2(&contents), 569)
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#;
        assert_eq!(solve_part_2(input), 4)
    }
}
