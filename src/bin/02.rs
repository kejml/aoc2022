use std::ops;
use crate::Result::{Lose, Draw, Win};
use crate::Shape::{Paper, Rock, Scissors};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    fn value(&self) -> u32 {
        match self {
            Win => { 6 }
            Draw => { 3 }
            Lose => { 0 }
        }
    }
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Rock => { 1 }
            Paper => { 2 }
            Scissors => { 3 }
        }
    }

    fn shape_from_result(&self, res: Result) -> Shape {
        match res {
            Lose => { match self {
                Rock => { Scissors }
                Paper => { Rock}
                Scissors => { Paper }
            } }
            Draw => { *self }
            Win => { match self {
                Rock => { Paper }
                Paper => { Scissors }
                Scissors => { Rock}
            }}
        }
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        match self {
            Rock => { Rock }
            Paper => { Paper }
            Scissors => { Scissors }
        }
    }
}

impl Copy for Shape {}

impl Clone for Result {
    fn clone(&self) -> Self {
        match self {
            Win => { Win }
            Draw => { Draw }
            Lose => { Lose }
        }
    }
}

impl Copy for Result {}

impl ops::Add<Shape> for Shape {
    type Output = u32;

    fn add(self, rhs: Shape) -> Self::Output {
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
        let match_result: Vec<Shape> = m.split(' ').map(|side| {
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

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input.split('\n').map(|line|{
        let shape1 = match line.chars().next().unwrap() {
            'A' => { Rock }
            'B' => { Paper }
            'C' => { Scissors }
            _ => panic!()
        };
        let result = match line.chars().nth(2).unwrap() {
            'X' => { Lose }
            'Y' => { Draw }
            'Z' => { Win }
            _ => panic!()
        };

        let shape2 = shape1.shape_from_result(result);
        result.value() + shape2.value()
    }).sum();

    Some(sum)

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
