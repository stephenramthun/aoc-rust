use std::collections::{HashMap, HashSet};
use std::{fs};

type Point = (i32, i32);
type Map = HashMap<Point, char>;

pub fn solver() {
    println!("Day 6:");

    let input = fs::read_to_string("./src/years/y2024/day06.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    let map = parse_input(input);
    let start_position = get_start_position(&map);
    get_visited_positions(&map, start_position).len()
}

fn solve_part_2(input: &str) -> usize {
    let map = parse_input(input);
    let start_position = get_start_position(&map);
    let visited_positions = get_visited_positions(&map, start_position);
    visited_positions
        .iter()
        .filter(|pos| {
            let map = with_obstacle(map.clone(), **pos);
            is_loop(&map, start_position)
        })
        .count()
}

fn with_obstacle(mut map: Map, at: Point) -> Map {
    map.insert(at, '#');
    map
}

fn is_loop(map: &Map, start_position: Point) -> bool {
    let mut current_position = start_position.clone();
    let mut current_direction = Direction::North;
    let mut visited = HashSet::new();

    loop {
        let next_position = go_forward(&current_direction, current_position);

        match map.get(&next_position) {
            None => return false,
            Some(char) => {
                if char == &'#' {
                    if visited.contains(&(current_position, current_direction)) {
                        return true;
                    }
                    visited.insert((current_position, current_direction));
                    current_direction = turn(&current_direction);
                } else {
                    current_position = next_position;
                }
            }
        }
    }
}

fn get_visited_positions(map: &Map, start_position: Point) -> HashSet<Point> {
    let mut current_position = start_position.clone();
    let mut current_direction = Direction::North;
    let mut visited: HashSet<Point> = HashSet::new();

    while map.contains_key(&current_position) {
        visited.insert(current_position);
        let next_position = go_forward(&current_direction, current_position);

        if map.get(&next_position) == Some(&'#') {
            current_direction = turn(&current_direction);
        } else {
            current_position = next_position;
        }
    }

    visited
}

fn turn(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn go_forward(direction: &Direction, (x, y): Point) -> Point {
    match direction {
        Direction::North => (x, y - 1),
        Direction::East => (x + 1, y),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
    }
}

fn parse_input(input: &str) -> Map {
    let mut map: Map = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map.insert((x as i32, y as i32), c);
        })
    });
    map
}

fn get_start_position(map: &Map) -> Point {
    match map.iter().enumerate().find(|(_, (_, &char))| char == '^') {
        Some((_, (point, _))) => point.clone(),
        None => panic!("Could not find start position"),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day06::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            .trim();

        assert_eq!(solve_part_1(input), 41);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day06.txt").unwrap();
        assert_eq!(solve_part_1(&input), 4515);
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            .trim();

        assert_eq!(solve_part_2(input), 6);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day06.txt").unwrap();
        assert_eq!(solve_part_2(&input), 1309);
    }
}
