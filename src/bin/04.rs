use std::ops::Range;

// TODO Range is not the best fit - upper bound is exclusive
pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split('\n').filter(|l| {!l.is_empty()})
        .map(|line| {
        let ranges: Vec<Range<u32>> = line.split(',').map(|range: &str| {
            let (l, r) = range.split_once('-').unwrap();
            l.parse::<u32>().unwrap()..(r.parse::<u32>().unwrap() + 1)
        }).collect();
        if ranges[0].contains(&ranges[1].start) && ranges[0].contains(&(ranges[1].end - 1))
            || ranges[1].contains(&ranges[0].start) && ranges[1].contains(&(ranges[0].end - 1)) {
            1
        } else {
            0
        }
    }).sum();
    Some(result)
}

//TODO duplicate
pub fn part_two(input: &str) -> Option<u32> {

    let result = input.split('\n').filter(|l| {!l.is_empty()})
        .map(|line| {
            let ranges: Vec<Range<u32>> = line.split(',').map(|range: &str| {
                let (l, r) = range.split_once('-').unwrap();
                l.parse::<u32>().unwrap()..(r.parse::<u32>().unwrap() + 1)
            }).collect();
            if ranges[0].contains(&ranges[1].start) || ranges[0].contains(&(ranges[1].end - 1))
                || ranges[1].contains(&ranges[0].start) || ranges[1].contains(&(ranges[0].end - 1)) {
                1
            } else {
                0
            }
        }).sum();
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
