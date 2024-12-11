use std::collections::{HashMap, HashSet};

type Stone = usize;

fn solve_part_1(input: &str, blinks: usize) -> usize {
    let mut stones: Vec<usize> = input.split(" ").map(|x| x.parse().unwrap()).collect();
    let mut count = 0;

    let mut map: HashMap<Stone, HashSet<Stone>> = HashMap::new();

    while count < blinks {
        count += 1;
        let len = stones.len();

        let mut i = 0;
        while i < len {
            let stone = stones[i];

            let res = map.entry(stone).or_insert(process_stone(stone));
            
            if stone == 0 {
                stones[i] = 1;
            } else if length(stone, 10) % 2 == 0 {
                let (a, b) = split(stone);
                stones[i] = a;
                stones.push(b);
            } else {
                stones[i] = stones[i] * 2024;
            }

            i += 1;
        }
    }

    stones.len()
}

fn process_stone(stone: Stone) -> HashSet<Stone> {
    let mut set = HashSet::new();
    if stone == 0 {
        set.insert(1);
    } else if length(stone, 10) % 2 == 0 {
        let (a, b) = split(stone);
        set.insert(a);
        set.insert(b);
    } else {
        set.insert(stone * 2024);
    }
    set
}

fn split(value: usize) -> (usize, usize) {
    let string = value.to_string();
    let (a, b) = string.split_at(string.len() / 2);
    (a.parse().unwrap(), b.parse().unwrap())
}

fn length(n: usize, base: usize) -> usize {
    let mut power = base;
    let mut count = 1;
    while n >= power {
        count += 1;
        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;
        } else {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day11::solve_part_1;

    #[test]
    fn test_input_part_1() {
        let input = "125 17";
        assert_eq!(solve_part_1(input, 6), 22);
        assert_eq!(solve_part_1(input, 25), 55312);
    }

    #[test]
    fn test_part_1() {
        let input = "0 27 5409930 828979 4471 3 68524 170";
        assert_eq!(solve_part_1(input, 25), 194482);
    }

    #[test]
    fn test_part_2() {
        let input = "0 27 5409930 828979 4471 3 68524 170";
        assert_eq!(solve_part_1(input, 75), 194482);
    }
}
