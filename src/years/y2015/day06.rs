type Point = (usize, usize);
type Operation = fn(usize) -> usize;

fn solve_part_1(input: &str) -> usize {
    let commands = parse_input(input, to_operation_part_1);
    let mut grid = make_grid();

    for command in &commands {
        for y in command.1 .1..=command.2 .1 {
            for x in command.1 .0..=command.2 .0 {
                grid[y][x] = command.0(grid[y][x])
            }
        }
    }

    grid.iter()
        .fold(0, |sum, row| sum + row.iter().filter(|x| **x == 1).count())
}

fn solve_part_2(input: &str) -> usize {
    let commands = parse_input(input, to_operation_part_2);
    let mut grid = make_grid();

    for command in &commands {
        for y in command.1 .1..=command.2 .1 {
            for x in command.1 .0..=command.2 .0 {
                grid[y][x] = command.0(grid[y][x])
            }
        }
    }

    grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |sum, cell| sum + cell)
    })
}

fn make_grid() -> Vec<Vec<usize>> {
    vec![vec![0; 1000]; 1000]
}

fn parse_input(
    input: &str,
    operation_mapper: fn(&str) -> Operation,
) -> Vec<(Operation, Point, Point)> {
    input
        .lines()
        .map(|line| match line.split(' ').collect::<Vec<_>>()[..] {
            [_, state, from, _, to] => (operation_mapper(state), to_point(from), to_point(to)),
            [state, from, _, to] => (operation_mapper(state), to_point(from), to_point(to)),
            _ => unreachable!(),
        })
        .collect()
}

fn to_operation_part_1(string: &str) -> Operation {
    match string {
        "off" => |_| 0,
        "on" => |_| 1,
        "toggle" => |x| if x == 1 { 0 } else { 1 },
        _ => unreachable!(),
    }
}

fn to_operation_part_2(string: &str) -> Operation {
    match string {
        "off" => |x| if x == 0 { 0 } else { x - 1 },
        "on" => |x| x + 1,
        "toggle" => |x| x + 2,
        _ => unreachable!(),
    }
}

fn to_point(string: &str) -> Point {
    let (x, y) = string.split_once(',').unwrap();
    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::years::y2015::day06::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = fs::read_to_string("./src/years/y2015/day06.txt").unwrap();
        assert_eq!(solve_part_1(&input), 400410);
    }

    #[test]
    fn test_input_part_2() {
        let input = fs::read_to_string("./src/years/y2015/day06.txt").unwrap();
        assert_eq!(solve_part_2(&input), 15343601);
    }
}
