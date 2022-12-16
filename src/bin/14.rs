#![feature(array_windows)]

extern crate core;

use std::collections::HashMap;
use crate::Tile::{Rock, Sand};

pub enum Tile {
    Rock,
    Sand,
}

pub fn sand_next(map: &HashMap<(usize, usize), Tile>, x: usize, y: usize) -> Option<(usize, usize)> {
    if map.get(&(x, y + 1)).is_none() {
        Some((x, y + 1))
    } else if map.get(&(x - 1, y + 1)).is_none() {
        Some((x - 1, y + 1))
    } else if map.get(&(x + 1, y + 1)).is_none() {
        Some((x + 1, y + 1))
    } else {
        None
    }
}

fn map_parse(input: &str) -> HashMap<(usize, usize), Tile> {
    let mut map = HashMap::<(usize, usize), Tile>::new();
    // TODO very crude parsing
    input.split('\n').for_each(|line| {
        line.split(" -> ").collect::<Vec<_>>()[..].windows(2).for_each(|w| {
            let point1 = w[0].split(',').map(|s| { s.parse::<usize>().unwrap() }).collect::<Vec<_>>();
            let point2 = w[1].split(',').map(|s| { s.parse::<usize>().unwrap() }).collect::<Vec<_>>();
            if point1[0] == point2[0] {
                if point1[1] > point2[1] {
                    for i in point2[1]..=point1[1] {
                        map.insert((point1[0], i), Rock);
                    }
                } else {
                    for i in point1[1]..=point2[1] {
                        map.insert((point1[0], i), Rock);
                    }
                };
            } else if point1[1] == point2[1] {
                if point1[0] > point2[0] {
                    for i in point2[0]..=point1[0] {
                        map.insert((i, point1[1]), Rock);
                    }
                } else {
                    for i in point1[0]..=point2[0] {
                        map.insert((i, point1[1]), Rock);
                    }
                };
            } else {
                panic!("Unexpected input");
            };
        });
    });
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = map_parse(input);

    let max_depth = map.keys().map(|k| { k.1 }).max().unwrap();

    let mut sands = 0;

    'new_sand: loop {
        sands += 1;
        let mut prev_send = (500, 0);
        while let Some(next) = sand_next(&map, prev_send.0, prev_send.1) {
            prev_send = next;
            if next.1 > max_depth {
                break 'new_sand;
            }
        }
        map.insert(prev_send, Sand);
    }

    Some(sands - 1)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut map = map_parse(input);

    let max_depth = map.keys().map(|k| { k.1 }).max().unwrap();

    let mut sands = 0;

    loop {
        sands += 1;
        let mut prev_send = (500, 0);
        while let Some(next) = sand_next(&map, prev_send.0, prev_send.1) {
            prev_send = next;
            if next.1 > max_depth {
                map.insert(prev_send, Sand);
                break;
            }
        }
        map.insert(prev_send, Sand);
        if prev_send == (500, 0) {
            break
        }
    }
    Some(sands)
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
