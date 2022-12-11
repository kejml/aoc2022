pub fn part_one(input: &str) -> Option<u32> {
    let mut cycles = 0;
    let mut x = 1;
    let result: i32 = input.lines().map(|line| {
        let mut cycle_result: i32;
        if ((cycles + 21) % 40) == 0 {
            cycle_result = x * (cycles + 1);
        } else {
            cycle_result = 0;
        }


        let mut instruction = line.split(' ');
        if instruction.next().unwrap() == "noop" {
            cycles += 1;
        } else {
            cycles += 2;
            let diff = instruction.next().unwrap().parse::<i32>().unwrap();
            if ((cycles + 20) % 40 ) == 0 {
                cycle_result = x * cycles;
            }
            x += diff;
        }

        cycle_result
    }).sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
