use std::collections::{HashMap, HashSet, VecDeque};

pub fn parse(input: &str) -> Vec<Vec<i8>> {
    let map = input.lines().map(|line| {
        line.chars().map(|ch| {
            match ch {
                'S' => {0}
                'E' => {27}
                _ => (ch as u8 - 96) as i8
            }
        }).collect::<Vec<i8>>()
    }).collect::<Vec<Vec<_>>>();
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);

    let height = map.len();
    let width = map[0].len();


    let mut start = (0,0);
    let mut end = (0,0);
    for j in 0..height {
        for i in 0..width {
            if map[j][i] == 0 {
                start = (j,i);
                map[j][i] = 1;
            } else if map[j][i] == 27 {
                end = (j,i);
                map[j][i] = 26;
            }
        }
    };

    Some(bfs(&map, start, end) as u32)
}

fn bfs (map: &Vec<Vec<i8>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut queue = VecDeque::<(usize, usize)>::new();
    let mut explored = HashSet::<(usize, usize)>::new();
    let mut path = HashMap::<(usize, usize), (usize, usize)>::new();

    explored.insert(start);
    queue.push_back(start);


    while !queue.is_empty() {
        let position = queue.pop_front().unwrap();
        if position == end {
            let height = map.len();
            let width = map[0].len();
            print_path(width, height, &map, &path, end);
            return count_length(&path, end);
        }
        for next in neighbours(position).iter().filter(|n| {is_accessible(map, position, **n)}) {
            if !explored.contains(&next) {
                explored.insert(*next);
                queue.push_back(*next);
                path.insert(*next, position);
            }
        }
    }
    -1

}

fn count_length(path: &HashMap::<(usize, usize), (usize, usize)>, end: (usize, usize)) -> i32 {
    let mut pos = Some(&end);
    let mut length = 0;
    while pos.is_some() {
        pos = path.get(&pos.unwrap());
        length += 1;
    }
    length -1
}

fn print_path(width: usize, height: usize, map: &Vec<Vec<i8>>, path: &HashMap::<(usize, usize), (usize, usize)>, end: (usize, usize)) {

    let mut visual_path = vec![vec![' '; width]; height];
    let mut pos = Some(&end);
    while pos.is_some() {
        visual_path[pos.unwrap().0][pos.unwrap().1] = (map[pos.unwrap().0][pos.unwrap().1] + 96) as u8 as char;
        pos = path.get(&pos.unwrap());
    }

    for j in 0..height {
        for i in 0..width {
            print!("{}", visual_path[j][i]);
        }
        println!();
    };
}

fn neighbours(position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![(position.0 + 1, position.1),  (position.0, position.1 + 1)];
    if position.0 > 0 {
        neighbours.push((position.0 - 1, position.1))
    }
    if position.1 > 0 {
        neighbours.push((position.0, position.1 - 1))
    }

    neighbours
}

fn is_accessible(map: &Vec<Vec<i8>>, position: (usize, usize), next: (usize, usize)) -> bool {
    let height = map.len();
    let width = map[0].len();

    if next.0 >= height || next.1 >= width || (map[next.0][next.1] - map[position.0][position.1]) > 1 {
        false
    } else {
        true
    }
}


pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
