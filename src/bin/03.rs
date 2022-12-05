use std::collections::HashSet;
use std::ops::Deref;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split('\n')
        .filter(|l| {
            !l.is_empty()
        })
        .map(|line| {
            let left = line[0..(line.len() / 2)].as_bytes().iter().collect::<HashSet<_>>();
            let right = line[line.len() / 2..line.len()].as_bytes().iter().collect::<HashSet<_>>();
            let res = left.intersection(&right).copied().last();
            res.unwrap()
        }).map(|c| {
        let r = if *c < 91u8 {
            *c - 38
        } else {
            *c - 96
        };
        r as u32
    }).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
