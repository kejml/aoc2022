#![feature(array_windows)]

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let res: u32 = input.as_bytes().windows(4).map_while(|w| {
        let window_set: HashSet<&u8> = w.iter().collect();
        if window_set.len() == 4 {
            None
        } else {
            Some(1)
        }
    }).sum();
    Some(res + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
