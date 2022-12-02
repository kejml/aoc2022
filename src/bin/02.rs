use std::ops;
use crate::Options::{Paper, Rock, Scissors};

#[derive(Debug)]
enum Options {
    Rock,
    Paper,
    Scissors,
}



impl Clone for Options {
    fn clone(&self) -> Self {
        match self {
            Rock => { Rock }
            Paper => { Paper }
            Scissors => { Scissors }
        }
    }
}

impl Copy for Options{}

impl ops::Add<Options> for Options {
    type Output = u32;

    fn add(self, rhs: Options) -> Self::Output {
        match self {
            Rock => {
                match rhs {
                    Rock => { 4 }
                    Paper => { 8 }
                    Scissors => { 3 }
                }
            }
            Paper => {
                match rhs {
                    Rock => { 1 }
                    Paper => { 5 }
                    Scissors => { 9 }
                }
            }
            Scissors => {
                match rhs {
                    Rock => { 7 }
                    Paper => { 2 }
                    Scissors => { 6 }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let results: u32 = input.split('\n').map(|m| {
        let match_result: Vec<Options> = m.split(' ').map(|side| {
            match side {
                "A" | "X" => { Rock }
                "B" | "Y" => { Paper }
                "C" | "Z" => { Scissors }
                _ => panic!()  // TODO handle
            }
        }).collect();
        match_result
    }).map(|line| {
        line[0] + line[1]
    }).sum();
    Some(results)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
