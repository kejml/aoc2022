use std::iter::Map;
use std::str::Split;

pub fn part_one(input: &str) -> Option<u32> {
    return calories_per_elf(input).max();
}

fn calories_per_elf(input: &str) -> Map<Split<&str>, fn(&str) -> u32> {
    input.split("\n\n").map(|elf| {
        let calories = elf.split('\n').map(|food| {
            food.parse::<u32>().expect("Invalid input")
        }).sum();
        calories
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut maxs = [0, 0, 0];
    calories_per_elf(input).for_each(|c| {
        if c > maxs[0] {
            maxs[2] = maxs[1];
            maxs[1] = maxs[0];
            maxs[0] = c;
        } else if c > maxs[1] {
            maxs[2] = maxs[1];
            maxs[1] = c;
        } else if c > maxs[2]  {
            maxs[2] = c
        }
    });

    Some(maxs.iter().sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
