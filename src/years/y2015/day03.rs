use std::collections::HashMap;
use std::convert::Infallible;
use std::{fs, ops};
use std::str::FromStr;

pub fn solver() {
    println!("Day 3:");

    let input = fs::read_to_string("./src/years/y2015/day03.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    let mut houses: HashMap<Position, usize> = HashMap::new();
    let mut current_position = Position { x: 0, y: 0 };

    houses.insert(current_position.clone(), 1);

    for direction in input.chars() {
        let direction = Direction::from_str(&direction.to_string()).unwrap();
        current_position += direction.value();

        match houses.get(&current_position) {
            Some(entry) => {
                houses.insert(current_position.clone(), entry + 1);
            },
            None => {
                houses.insert(current_position.clone(), 1);
            }
        }
    }

    houses.len()
}

fn solve_part_2(input: &str) -> usize {
    let mut houses: HashMap<Position, usize> = HashMap::new();
    let mut current_position = Position { x: 0, y: 0 };

    houses.insert(current_position.clone(), 1);

    let binding = input.chars().collect::<Vec<_>>();
    let (a, b): (Vec<char>, Vec<char>) = binding.chunks(2).map(|c| (c[0], c[1])).unzip();

    for direction in a {
        let direction = Direction::from_str(&direction.to_string()).unwrap();
        current_position += direction.value();

        match houses.get(&current_position) {
            Some(entry) => {
                houses.insert(current_position.clone(), entry + 1);
            },
            None => {
                houses.insert(current_position.clone(), 1);
            }
        }
    }

    current_position = Position { x: 0, y: 0 };

    for direction in b {
        let direction = Direction::from_str(&direction.to_string()).unwrap();
        current_position += direction.value();

        match houses.get(&current_position) {
            Some(entry) => {
                houses.insert(current_position.clone(), entry + 1);
            },
            None => {
                houses.insert(current_position.clone(), 1);
            }
        }
    }

    houses.len()
}

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn value(self) -> Position {
        match self {
            Direction::North => Position { x: 0, y: -1 },
            Direction::East => Position { x: 1, y: 0 },
            Direction::South => Position { x: 0, y: 1 },
            Direction::West => Position { x: -1, y: 0 },
        }
    }
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::North),
            ">" => Ok(Direction::East),
            "v" => Ok(Direction::South),
            "<" => Ok(Direction::West),
            _ => panic!("bad direction: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::years::y2015::day03::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2015/day03.txt").unwrap();
        assert_eq!(solve_part_1(&input), 2081);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2015/day03.txt").unwrap();
        assert_eq!(solve_part_2(&input), 2341);
    }
}
