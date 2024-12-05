use std::fs;

pub fn solver() {
    println!("Day 2:");

    let input = fs::read_to_string("./src/years/y2015/day02.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    parse_input(input).iter().fold(0, |sum, gift| sum + gift.area() + gift.smallest_side())
}

fn solve_part_2(input: &str) -> usize {
    parse_input(input).iter().fold(0, |sum, gift| sum + gift.length_of_ribbon())
}

fn parse_input(input: &str) -> Vec<GiftBox> {
    input.split("\n").map(|d| {
        let components: Vec<usize> = d.split('x').map(|d| d.parse().unwrap()).collect();
        GiftBox {
            length: components[0],
            width: components[1],
            height: components[2],
        }
    }).collect()
}

struct GiftBox {
    length: usize,
    width: usize,
    height: usize,
}

impl GiftBox {
    fn area(&self) -> usize {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn smallest_side(&self) -> usize {
        *vec![self.length * self.width, self.width * self.height, self.height * self.length].iter().min().unwrap()
    }

    fn smallest_perimeter(&self) -> usize {
        *vec![self.length + self.width, self.width + self.height, self.height + self.length].iter().min().unwrap() * 2
    }

    fn length_of_ribbon(&self) -> usize {
        self.smallest_perimeter() + self.length * self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::years::y2015::day02::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./src/years/y2015/day02.txt").unwrap();
        assert_eq!(solve_part_1(&input), 1598415);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./src/years/y2015/day02.txt").unwrap();
        assert_eq!(solve_part_2(&input), 3812909);
    }
}
