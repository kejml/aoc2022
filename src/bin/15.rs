use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::Tile::{Beacon, Covered, Sensor};

pub enum Tile {
    Sensor,
    Beacon,
    Covered,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Sensor => { 'S' }
            Beacon => { 'B' }
            Covered => { '#' }
        };
        write!(f, "{char}")
    }
}

pub fn parse_map(input: &str, significant: i32) -> HashMap<(i32, i32), Tile> {
    let mut map = HashMap::<(i32, i32), Tile>::new();
    input.lines().for_each(|line| {
        let line = line.split(' ').collect::<Vec<_>>();
        let sx = &(line[2])[2..line[2].len() - 1].parse::<i32>().unwrap();
        let sy = &(line[3])[2..line[3].len() - 1].parse::<i32>().unwrap();

        map.insert((*sx, *sy), Sensor);

        let bx = &(line[8])[2..line[8].len() - 1].parse::<i32>().unwrap();
        let by = &(line[9])[2..line[9].len()].parse::<i32>().unwrap();

        map.insert((*bx, *by), Beacon);

        let distance = (sx - bx).abs() + (sy - by).abs();


        for j in sx - distance..=sx + distance {
            if (sx - j).abs() + (sy - significant).abs() <= distance && map.get(&(j, significant)).is_none() {
                map.insert((j, significant), Covered);
            }
        }
    });
    map
}

pub fn debug_print(map: &HashMap<(i32, i32), Tile>) {
    let min_x = map.keys().map(|k| { k.0 }).min().unwrap();
    let max_x = map.keys().map(|k| { k.0 }).max().unwrap();
    let min_y = map.keys().map(|k| { k.1 }).min().unwrap();
    let max_y = map.keys().map(|k| { k.1 }).max().unwrap();
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            let char = match map.get(&(j, i)) {
                None => { '.' }
                Some(Sensor) => { 'S' }
                Some(Beacon) => { 'B' }
                Some(Covered) => { '#' }
            };
            print!("{char}")
        }
        println!()
    }
}

pub fn part_one_test(input: &str, significant: i32) -> Option<u32> {
    let map = parse_map(input, significant);
    let min_x = map.keys().map(|k| { k.0 }).min().unwrap();
    let max_x = map.keys().map(|k| { k.0 }).max().unwrap();

    // debug_print(&map);

    let mut covered = 0;
    for i in min_x..=max_x {
        if let Some(Covered) = map.get(&(i, significant)) {
            covered += 1;
        }
    }
    Some(covered)
}


pub fn part_one(input: &str) -> Option<u32> {
    part_one_test(input, 2000000)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one_test(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
