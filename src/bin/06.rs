#![feature(array_windows)]

use std::collections::HashSet;

fn find_packet(input: &str, packet_size: usize) -> Option<u32> {
    let res: u32 = input.as_bytes().windows(packet_size).map_while(|w| {
        let window_set: HashSet<&u8> = w.iter().collect();
        if window_set.len() == packet_size {
            None
        } else {
            Some(1)
        }
    }).sum();
    Some(res + (packet_size as u32))
}

pub fn part_one(input: &str) -> Option<u32> {
    find_packet(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_packet(input, 14)
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
        assert_eq!(part_two(&input), Some(19));
    }
}
