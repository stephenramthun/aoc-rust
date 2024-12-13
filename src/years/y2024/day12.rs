use crate::util::runner::Runner;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

pub fn runner(day: u8) -> Runner {
    let input = fs::read_to_string("./src/years/y2024/day12.txt").expect("File not found");
    Runner {
        day,
        part1: || 0,
        part2: || 0,
    }
}

const OVER: Point = (0, -1);
const UNDER: Point = (0, 1);
const RIGHT: Point = (1, 0);
const LEFT: Point = (-1, 0);
const DIRECTIONS: &'static [Point] = &[OVER, UNDER, RIGHT, LEFT];

fn part1(input: &str) -> usize {
    let (regions, _) = parse_input(input);

    regions.iter().fold(0, |sum, region| {
        sum + (region.area * region.perimeter) as usize
    })
}

fn part2(input: &str) -> usize {
    let (regions, garden_map) = parse_input(input);

    regions
        .iter()
        .fold(0, |sum, region| sum + (region.area * region.sides) as usize)
}

fn count_sides(region: &Region, garden_map: &Vec<Vec<char>>) -> usize {
    let mut sides = 0;
    // Count vertical sides
    for x in region.start.0..=region.end.0 {
        let mut right = false;
        let mut left = false;
        for y in region.start.1..=region.end.1 {
            let pos = (x, y);
            if !region.points.contains(&pos) {
                right = false;
                left = false;
                continue;
            }
            if is_side(&pos, &RIGHT, &garden_map) {
                if !right {
                    sides += 1;
                    right = true;
                }
            } else {
                right = false
            }
            if is_side(&pos, &LEFT, &garden_map) {
                if !left {
                    sides += 1;
                    left = true;
                }
            } else {
                left = false
            }
        }
    }
    // Count horizontal sides
    for y in region.start.1..=region.end.1 {
        let mut over = false;
        let mut under = false;
        for x in region.start.0..=region.end.0 {
            let pos = (x, y);
            if !region.points.contains(&pos) {
                over = false;
                under = false;
                continue;
            }
            if is_side(&pos, &OVER, &garden_map) {
                if !over {
                    sides += 1;
                    over = true;
                }
            } else {
                over = false
            }
            if is_side(&pos, &UNDER, &garden_map) {
                if !under {
                    sides += 1;
                    under = true;
                }
            } else {
                under = false
            }
        }
    }
    sides
}

fn is_side(pos: &Point, direction: &Point, garden_map: &Vec<Vec<char>>) -> bool {
    let delta = add_points(pos, direction);
    out_of_bounds(&delta, garden_map)
        || garden_map[delta.1 as usize][delta.0 as usize]
            != garden_map[pos.1 as usize][pos.0 as usize]
}

fn build_region(
    mut current_region: &mut Region,
    current_pos: &Point,
    visited: &mut HashSet<Point>,
    matrix: &Vec<Vec<char>>,
) {
    if out_of_bounds(current_pos, matrix)
        || visited.contains(current_pos)
        || matrix[current_pos.1 as usize][current_pos.0 as usize] != current_region.name
    {
        return;
    }

    visited.insert(current_pos.clone());

    current_region.perimeter += perimeter_for_plot(current_pos, current_region.name, matrix);
    current_region.area += 1;
    current_region.points.insert(current_pos.clone());

    current_region.start = take_smallest(current_pos, &current_region.start);
    current_region.end = take_largest(current_pos, &current_region.end);

    DIRECTIONS
        .iter()
        .map(|dir| add_points(dir, current_pos))
        .for_each(|pos| build_region(&mut current_region, &pos, visited, matrix))
}

fn perimeter_for_plot(plot_pos: &Point, plot_name: char, garden_map: &Vec<Vec<char>>) -> u32 {
    let deltas = DIRECTIONS.iter().map(|dir| add_points(dir, plot_pos));

    deltas
        .clone()
        .filter(|pos| !out_of_bounds(pos, garden_map))
        .filter(|pos| garden_map[pos.1 as usize][pos.0 as usize] != plot_name)
        .count() as u32
        + deltas.filter(|pos| out_of_bounds(pos, garden_map)).count() as u32
}

fn parse_input(input: &str) -> (Vec<Region>, Vec<Vec<char>>) {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            let current_pos = (x as i32, y as i32);
            if !visited.contains(&current_pos) {
                // Found new region, consume entire region and save to result vec
                let mut region = Region {
                    name: char,
                    perimeter: 0,
                    area: 0,
                    start: current_pos,
                    end: current_pos,
                    sides: 0,
                    points: HashSet::new(),
                };
                build_region(&mut region, &current_pos, &mut visited, &matrix);
                regions.push(region);
            }
        }
    }

    (
        regions
            .iter()
            .map(|region| Region {
                name: region.name,
                perimeter: region.perimeter,
                area: region.area,
                sides: count_sides(&region, &matrix) as u32,
                start: region.start,
                end: region.end,
                points: region.points.clone(),
            })
            .collect(),
        matrix,
    )
}

type Point = (i32, i32);

fn add_points(a: &Point, b: &Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn out_of_bounds<T>(pos: &Point, map: &Vec<Vec<T>>) -> bool {
    pos.1 < 0 || pos.0 < 0 || pos.1 >= map.len() as i32 || pos.0 >= map[0].len() as i32
}

fn take_smallest(a: &Point, b: &Point) -> Point {
    (min(a.0, b.0), min(a.1, b.1))
}

fn take_largest(a: &Point, b: &Point) -> Point {
    (max(a.0, b.0), max(a.1, b.1))
}

#[derive(Debug)]
struct Region {
    name: char,
    perimeter: u32,
    area: u32,
    sides: u32,
    start: Point,
    end: Point,
    points: HashSet<Point>,
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day12::{count_sides, parse_input, part1, part2, perimeter_for_plot};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = r#"
AAAA
BBCD
BBCC
EEEC"#
            .trim();
        assert_eq!(part1(input), 140);
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
AAAA
BBCD
BBCC
EEEC"#
            .trim();
        assert_eq!(part2(input), 80);
    }

    #[test]
    fn test_case_2_part_2() {
        let input = r#"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#
            .trim();
        assert_eq!(part2(input), 236);
    }

    #[test]
    fn test_case_3_part_2() {
        let input = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
            .trim();
        assert_eq!(part2(input), 1206);
    }

    #[test]
    fn test_perimeter_for_plot() {
        let garden_map = vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C'],
        ];

        assert_eq!(perimeter_for_plot(&(0, 0), 'A', &garden_map), 3);
        assert_eq!(perimeter_for_plot(&(1, 0), 'A', &garden_map), 2);
        assert_eq!(perimeter_for_plot(&(2, 0), 'A', &garden_map), 2);
        assert_eq!(perimeter_for_plot(&(3, 0), 'A', &garden_map), 3);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day12.txt").unwrap();
        assert_eq!(part1(&input), 1363484);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day12.txt").unwrap();
        assert_eq!(part2(&input), 838988);
    }
}
