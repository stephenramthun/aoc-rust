use anyhow::{anyhow, Result};
use std::{fs, ops};

pub fn solver() {
    println!("Day 4:");

    let input = fs::read_to_string("./src/years/y2024/day04.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

#[derive(Copy, Clone, Debug)]
struct Direction {
    x: isize,
    y: isize,
}

struct Position {
    x: usize,
    y: usize,
}

impl ops::Add<Direction> for Position {
    type Output = Result<Position>;

    fn add(self, rhs: Direction) -> Result<Position> {
        let (x, y) = (self.x, self.y);
        let new_x: usize = (x as isize + rhs.x).try_into()?;
        let new_y: usize = (y as isize + rhs.y).try_into()?;

        Ok(Position { x: new_x, y: new_y })
    }
}

struct Matrix {
    matrix: Vec<Vec<char>>,
}

impl Matrix {
    fn get_cell(&self, x: usize, y: usize) -> Result<char> {
        let row = self.get_row(y)?;
        match row.get(x) {
            Some(cell) => Ok(*cell),
            None => Err(anyhow!("X:{x} not found in matrix")),
        }
    }

    fn get_row(&self, y: usize) -> Result<Vec<char>> {
        match self.matrix.get(y) {
            Some(row) => Ok(row.to_vec()),
            None => Err(anyhow!("Row with y:{y} not found in matrix")),
        }
    }
}

fn solve_part_1(input: &str) -> usize {
    const DELTAS: [Direction; 8] = [
        Direction { x: 1, y: 0 },
        Direction { x: 1, y: 1 },
        Direction { x: 0, y: 1 },
        Direction { x: -1, y: 1 },
        Direction { x: -1, y: 0 },
        Direction { x: -1, y: -1 },
        Direction { x: 0, y: -1 },
        Direction { x: 1, y: -1 },
    ];

    let search_word = ['X', 'M', 'A', 'S'];
    let input: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();
    let matrix = Matrix { matrix: input };

    let mut matches = 0;

    for (y, row) in matrix.matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == search_word[0] {
                'directions: for delta in DELTAS.iter() {
                    for (i, char) in search_word.iter().enumerate() {
                        let position = Position { x, y };
                        let delta = Direction {
                            x: delta.x * i as isize,
                            y: delta.y * i as isize,
                        };

                        let new_position = match position + delta {
                            Ok(position) => position,
                            Err(_) => continue 'directions,
                        };

                        let cell = match matrix.get_cell(new_position.x, new_position.y) {
                            Ok(x) => x,
                            Err(_) => continue 'directions,
                        };

                        if *char != cell {
                            continue 'directions;
                        }

                        if i == search_word.len() - 1 {
                            matches += 1;
                        }
                    }
                }
            }
        }
    }

    matches
}

fn solve_part_2(input: &str) -> usize {
    const MASKS: [[Direction; 2]; 2] = [
        [Direction { x: -1, y: -1 }, Direction { x: 1, y: 1 }],
        [Direction { x: -1, y: 1 }, Direction { x: 1, y: -1 }],
    ];

    let input: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();
    let matrix = Matrix { matrix: input };

    let mut matches = 0;

    for y in 1..matrix.matrix.len() - 1 {
        let row = matrix.get_row(y).unwrap();
        for x in 1..row.len() - 1 {
            let cell = matrix.get_cell(x, y).unwrap();
            if cell == 'A' {
                let is_x_mas = MASKS.iter().all(|mask| {
                    let cells: Vec<char> = mask
                        .iter()
                        .map(|&direction| {
                            let position = Position { x, y };
                            let delta = (position + direction).unwrap();
                            matrix.get_cell(delta.x, delta.y).unwrap()
                        })
                        .collect();

                    cells == vec!['M', 'S'] || cells == vec!['S', 'M']
                });

                if is_x_mas {
                    matches += 1
                };
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day04::{solve_part_1, solve_part_2};
    use std::fs::read_to_string;

    #[test]
    fn test_input_part_1() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
            .trim();
        assert_eq!(solve_part_1(input), 18);
    }

    #[test]
    fn test_part_1() {
        let input = read_to_string("./src/years/y2024/day04.txt").unwrap();
        assert_eq!(solve_part_1(&input), 2618);
    }

    #[test]
    fn test_input_part_2() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
            .trim();
        assert_eq!(solve_part_2(input), 9);
    }

    #[test]
    fn test_part_2() {
        let input = read_to_string("./src/years/y2024/day04.txt").unwrap();
        assert_eq!(solve_part_2(&input), 2011);
    }
}
