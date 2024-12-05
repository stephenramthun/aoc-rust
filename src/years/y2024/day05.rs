use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fs;
use std::str::FromStr;

pub fn solver() {
    println!("Day 5:");

    let input = fs::read_to_string("./src/years/y2024/day05.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    Model::from_str(input)
        .unwrap()
        .valid_updates()
        .iter()
        .fold(0, |sum, update| sum + update.mid() as usize)
}

fn solve_part_2(input: &str) -> usize {
    let model = Model::from_str(input).unwrap();
    model
        .clone()
        .invalid_updates()
        .iter()
        .map(|update| make_valid(update, model.rules.clone()))
        .fold(0, |sum, update| sum + update.mid() as usize)
}

trait MiddleElement<T> {
    fn mid(&self) -> T;
}

impl MiddleElement<u8> for Vec<u8> {
    fn mid(&self) -> u8 {
        self[self.len() / 2]
    }
}

#[derive(Debug, Clone)]
struct Model {
    rules: HashMap<u8, HashSet<u8>>,
    updates: Vec<Vec<u8>>,
}

fn is_valid(update: &Vec<u8>, model: &Model) -> bool {
    for (i, val) in update.iter().enumerate() {
        let rules = match model.rules.get(val) {
            Some(rules) => rules,
            None => continue,
        };

        if rules.iter().any(|rule| update[i + 1..].contains(rule)) {
            return false
        }
    }
    true
}

fn make_valid(invalid_update: &Vec<u8>, rules: HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    let mut result = Vec::new();

    for val in invalid_update {
        let rules = match rules.get(val) {
            Some(rules) => rules,
            None => {
                result.insert(0, val.clone());
                continue
            },
        };
        let max_index = rules
            .iter()
            .map(|&rule| {
                match result.iter().position(|&x| x == rule) {
                    Some(index) => index + 1,
                    None => 0,
                }
            })
            .max()
            .unwrap();
        result.insert(max_index as usize, val.clone());
    }

    result
}

impl Model {
    fn invalid_updates(self) -> Vec<Vec<u8>> {
        self.updates
            .clone()
            .into_iter()
            .filter(|update| !is_valid(update, &self))
            .collect()
    }

    fn valid_updates(self) -> Vec<Vec<u8>> {
        self.updates
            .clone()
            .into_iter()
            .filter(|update| is_valid(update, &self))
            .collect()
    }
}

impl FromStr for Model {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, updates) = s.split_once("\n\n").unwrap();

        let rules = rules
            .split("\n")
            .map(|rule| {
                let (a, b) = rule.split_once("|").unwrap();
                (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap())
            })
            .fold(HashMap::new(), |mut map: HashMap<u8, HashSet<u8>>, rule| {
                if !map.contains_key(&rule.1) {
                    map.insert(rule.1, HashSet::new());
                }

                map.entry(rule.1).and_modify(|set| {
                    set.insert(rule.0);
                });
                map
            });

        let updates = updates
            .split("\n")
            .map(|update| {
                update
                    .split(",")
                    .map(|val| val.parse::<u8>().unwrap())
                    .collect()
            })
            .collect();

        Ok(Model { rules, updates })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::years::y2024::day05::{solve_part_1, solve_part_2};

    #[test]
    fn test_input_part_1() {
        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#
            .trim();

        assert_eq!(solve_part_1(input), 143)
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day05.txt").unwrap();
        assert_eq!(solve_part_1(&input), 5374)
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#
            .trim();

        assert_eq!(solve_part_2(input), 123)
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day05.txt").unwrap();
        assert_eq!(solve_part_2(&input), 4260)
    }
}
