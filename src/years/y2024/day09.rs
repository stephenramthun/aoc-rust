use std::collections::HashSet;
use std::fs;

pub fn solver() {
    println!("Day 9:");

    let input = fs::read_to_string("./src/years/y2024/day09.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    let blocks = to_blocks(input);
    let compacted = compact(blocks);
    compacted
        .iter()
        .enumerate()
        .fold(0, |checksum, (i, id)| checksum + i * id)
}

fn solve_part_2(input: &str) -> usize {
    let blocks = to_blocks(input);
    let compacted = compact_2(blocks);
    compacted.iter().enumerate().fold(0, |checksum, (i, id)| {
        checksum
            + match id.as_str() {
                "." => 0,
                id => id.parse::<usize>().unwrap() * i,
            }
    })
}

fn compact(blocks: Vec<String>) -> Vec<usize> {
    let mut i = 0;
    let mut j = blocks.len() - 1;
    let mut result: Vec<usize> = vec![];

    loop {
        if i > j {
            break;
        }

        match blocks[i].as_str() {
            "." => {
                while blocks[j] == "." {
                    j -= 1;
                }
                result.push(blocks[j].parse::<usize>().unwrap());
                j -= 1;
            }
            c => result.push(c.parse::<usize>().unwrap()),
        }

        i += 1
    }

    result
}

fn compact_2(blocks: Vec<String>) -> Vec<String> {
    let mut cursor = blocks.len() - 1;
    let mut result: Vec<String> = blocks.clone();
    let mut visited = HashSet::new();

    while cursor > 0 {
        // Find end of file to move
        while result[cursor] == "." {
            cursor -= 1;
        }

        // Find start and end index of file
        let end_index = cursor;
        let current_id = blocks[cursor].clone();

        while cursor > 0 && blocks[cursor - 1] == current_id.as_str() {
            cursor -= 1;
        }

        if visited.contains(&current_id) {
            cursor = if cursor > 0 { cursor - 1 } else { 0 };
            continue;
        }

        visited.insert(current_id.clone());

        // Find leftmost slot of free space that fits the file
        let file_size = end_index - cursor;
        let mut free_slot_start = 0;

        while free_slot_start < cursor {
            while free_slot_start < cursor && result[free_slot_start] != "." {
                free_slot_start += 1;
            }

            let mut free_slot_end = free_slot_start;
            while free_slot_end < cursor && result[free_slot_end + 1] == "." {
                free_slot_end += 1;
            }

            if (free_slot_end - free_slot_start) >= file_size {
                // Move
                for i in 0..=file_size {
                    result.swap(free_slot_start + i, cursor + i);
                }
                break;
            }

            free_slot_start = free_slot_end + 1;
        }

        if cursor == 0 {
            break;
        }

        cursor -= 1;
    }

    result
}

fn to_blocks(input: &str) -> Vec<String> {
    input
        .chars()
        .enumerate()
        .map(|(i, char)| {
            let digit = char.to_digit(10).unwrap();

            if i % 2 != 0 {
                vec![".".to_string(); digit as usize]
            } else {
                let id = format!("{}", i / 2);
                vec![id.to_string(); digit as usize]
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::years::y2024::day09::{solve_part_1, solve_part_2};
    use std::fs;

    #[test]
    fn test_input_part_1() {
        let input = "2333133121414131402";
        assert_eq!(solve_part_1(input), 1928);
    }

    #[test]
    fn test_input_part_2() {
        let input = "2333133121414131402";
        assert_eq!(solve_part_2(input), 2858);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2024/day09.txt").unwrap();
        assert_eq!(solve_part_1(&input), 6334655979668);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2024/day09.txt").unwrap();
        assert_eq!(solve_part_2(&input), 6349492251099);
    }
}
