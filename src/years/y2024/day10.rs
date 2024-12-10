use std::collections::HashSet;
use std::fs;

pub fn solver() {
    println!("Day 10:");

    let input = fs::read_to_string("./src/years/y2024/day09.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

type Point = (i32, i32);

fn get_value(map: &Vec<Vec<u32>>, position: Point) -> u32 {
    map[position.1 as usize][position.0 as usize]
}

fn add_points(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn out_of_bounds(map: &Vec<Vec<u32>>, position: Point) -> bool {
    let (x, y) = position;
    x < 0 || y < 0 || x >= map.len() as i32 || y >= map[0].len() as i32
}

fn get_visited_points(
    map: &Vec<Vec<u32>>,
    visited: &mut HashSet<Point>,
    point: Point,
    last_value: i32,
) {
    if out_of_bounds(map, point) {
        return;
    }

    let current_value = get_value(map, point);

    if current_value != (last_value + 1) as u32 {
        return;
    }

    visited.insert(point);

    if current_value == 9 {
        return;
    }

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .map(|dir| add_points(dir, point))
        .into_iter()
        .filter(|pos| !out_of_bounds(map, *pos))
        .for_each(|pos| get_visited_points(map, visited, pos, current_value as i32))
}

fn get_trailhead_rating(map: &Vec<Vec<u32>>, point: Point, last_value: i32) -> usize {
    if out_of_bounds(map, point) {
        return 0;
    }

    let current_value = get_value(map, point);

    if current_value != (last_value + 1) as u32 {
        return 0;
    }

    if current_value == 9 {
        return 1;
    }

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .map(|dir| add_points(dir, point))
        .into_iter()
        .filter(|pos| !out_of_bounds(map, *pos))
        .fold(0, |sum, pos| {
            sum + get_trailhead_rating(map, pos, current_value as i32)
        })
}

fn solve_part_1(input: &str) -> usize {
    let input = parse_input(input);
    let mut scores = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                let mut visited = HashSet::new();
                get_visited_points(&input, &mut visited, (x as i32, y as i32), -1);
                scores.push(
                    visited
                        .iter()
                        .filter(|(x, y)| input[*y as usize][*x as usize] == 9)
                        .count(),
                );
            }
        }
    }

    scores.iter().sum()
}

fn solve_part_2(input: &str) -> usize {
    let input = parse_input(input);
    let mut scores = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                scores.push(get_trailhead_rating(&input, (x as i32, y as i32), -1));
            }
        }
    }

    scores.iter().sum()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day10::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
            .trim();
        assert_eq!(solve_part_1(input), 36);
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
            .trim();
        assert_eq!(solve_part_2(input), 81);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day10.txt").unwrap();
        assert_eq!(solve_part_1(&input), 659);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day10.txt").unwrap();
        assert_eq!(solve_part_2(&input), 1463);
    }
}
