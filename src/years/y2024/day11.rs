use std::collections::HashMap;
use std::iter::successors;

type Stone = usize;

pub fn solver() {
    println!("Day 10:");

    let input = "0 27 5409930 828979 4471 3 68524 170";
    println!("Part 1: {}", solve(&input, 25));
    println!("Part 2: {}", solve(&input, 75));
}

fn parse_input(input: &str) -> HashMap<Stone, usize> {
    input
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut map, stone| {
            *map.entry(stone).or_default() += 1;
            map
        })
}

fn solve(input: &str, blinks: usize) -> usize {
    successors(Some(parse_input(input)), |stones| {
        Some(process_stones(stones))
    })
    .nth(blinks)
    .unwrap()
    .values()
    .sum()
}

fn process_stones(stones: &HashMap<Stone, usize>) -> HashMap<Stone, usize> {
    let mut result = HashMap::new();

    for (stone, count) in stones {
        if *stone == 0 {
            *result.entry(1).or_default() += count;
        } else if stone.ilog10() % 2 == 1 {
            let (a, b) = split(*stone);
            *result.entry(a).or_default() += count;
            *result.entry(b).or_default() += count;
        } else {
            *result.entry(*stone * 2024).or_default() += count;
        }
    }

    result
}
fn split(value: usize) -> (usize, usize) {
    let div = 10usize.pow((value.ilog10() + 1) / 2);
    (value / div, value % div)
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day11::solve;

    #[test]
    fn test_input_part_1() {
        let input = "125 17";
        assert_eq!(solve(input, 6), 22);
        assert_eq!(solve(input, 25), 55312);
    }

    #[test]
    fn test_part_1() {
        let input = "0 27 5409930 828979 4471 3 68524 170";
        assert_eq!(solve(input, 25), 194482);
    }

    #[test]
    fn test_part_2() {
        let input = "0 27 5409930 828979 4471 3 68524 170";
        assert_eq!(solve(input, 75), 232454623677743);
    }
}
