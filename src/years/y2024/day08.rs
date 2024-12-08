use std::collections::HashMap;
use std::fs;

pub fn solver() {
    println!("Day 8:");

    let input = fs::read_to_string("./src/years/y2024/day08.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

type Point = (i32, i32);
type Grid = HashMap<Point, char>;
type Antennas = HashMap<char, Vec<Point>>;

fn solve_part_1(input: &str) -> usize {
    let (mut grid, antennas) = parse_input(input);
    let mut num_antinodes = 0;

    for (_, points) in antennas.iter() {
        for (a, &point_a) in points.iter().enumerate() {
            for (b, &point_b) in points.iter().enumerate() {
                if a == b {
                    continue
                }
                let antinode = calculate_antinode(point_a, point_b);

                if grid.contains_key(&antinode) && grid[&antinode] != '#' {
                    grid.insert(antinode, '#');
                    num_antinodes += 1;
                }
            }
        }
    }

    num_antinodes
}

fn solve_part_2(input: &str) -> usize {
    let (mut grid, antennas) = parse_input(input);
    let mut num_antinodes = 0;

    for (_, points) in antennas.iter() {
        for (a, &point_a) in points.iter().enumerate() {
            num_antinodes += 1;
            for (b, &point_b) in points.iter().enumerate() {
                if a == b {
                    continue
                }
                let delta = calculate_antinode_delta(point_a, point_b);
                let mut current = add_points(delta, point_a);

                while grid.contains_key(&current) {
                    if grid[&current] == '.' {
                        grid.insert(current, '#');
                        num_antinodes += 1;
                    }
                    current = add_points(delta, current);
                }
            }
        }
    }

    num_antinodes
}

fn add_points(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn calculate_antinode(point: Point, other: Point) -> Point {
    let (diff_y, diff_x) = calculate_antinode_delta(point, other);
    (point.0 + diff_y, point.1 + diff_x)
}

fn calculate_antinode_delta(point: Point, other: Point) -> Point {
    let diff_y = point.0 - other.0;
    let diff_x = point.1 - other.1;

    (diff_y, diff_x)
}

fn parse_input(input: &str) -> (Grid, Antennas) {
    let mut grid = Grid::new();
    let mut antennas = Antennas::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.insert(point(x, y), char);

            match char {
                '.' => {}
                c => {
                    if antennas.contains_key(&c) {
                        antennas.get_mut(&c).unwrap().push(point(x, y));
                    } else {
                        antennas.insert(c, vec![point(x, y)]);
                    }
                }
            }
        }
    }

    (grid, antennas)
}

fn point(x: usize, y: usize) -> Point {
    (x as i32, y as i32)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::years::y2024::day08::{solve_part_1, solve_part_2};

    #[test]
    fn test_input_part_1() {
        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
            .trim();
        assert_eq!(solve_part_1(input), 14);
    }


    #[test]
    fn test_input_part_2() {
        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
            .trim();
        assert_eq!(solve_part_2(input), 34);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day08.txt").unwrap();
        assert_eq!(solve_part_1(&input), 254);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day08.txt").unwrap();
        assert_eq!(solve_part_2(&input), 951);
    }
}
