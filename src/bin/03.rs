#![feature(iter_array_chunks)]

use std::collections::HashSet;

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
        }).map(|c| {    //TOD reuse in part 2
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
    let result = input.split('\n')
        .filter(|l| {
            !l.is_empty()
        }).array_chunks().map(|chunk: [&str; 3]| {
        // TODO repeated code
        let c0 = chunk[0].as_bytes().iter().collect::<HashSet<_>>();
        let c1 = chunk[1].as_bytes().iter().collect::<HashSet<_>>();
        let c2 = chunk[2].as_bytes().iter().collect::<HashSet<_>>();
        let res = c0.intersection(&c1)
            .copied()
            .collect::<HashSet<&u8>>()
            .intersection(&c2)
            .copied()
            .last();
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
        assert_eq!(part_two(&input), Some(70));
    }
}
