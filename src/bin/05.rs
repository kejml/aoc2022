use std::str::Split;

fn read_result(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|stack| {
        stack.last().unwrap_or(&' ')
    }).collect::<Vec<&char>>().into_iter().collect()
}

fn parse_initial_state(split: &mut Split<&str>) -> Vec<Vec<char>> {
    let mut stacks_input = split.next().unwrap().lines().collect::<Vec<_>>();
    let number_of_stacks: usize = stacks_input.pop().unwrap().split(" ").last().unwrap().parse().unwrap();

    stacks_input.reverse();

    let mut stacks: Vec<Vec<char>> = vec![vec![' '; 0]; number_of_stacks];
    for s in stacks_input {
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..number_of_stacks {
            s.get(i * 4 + 1).filter(|c| { !c.is_ascii_whitespace() }).and_then(|container| {
                let _ = &stacks[i].push(*container);
                Some(container)
            });
        }
    }
    stacks
}

fn move_containers_9000(instructions: &mut Split<&str>, stacks: &mut Vec<Vec<char>>) {
    instructions.next().unwrap().lines().for_each(|line| {
        let instruction = line.split(' ').collect::<Vec<_>>();
        let times: u8 = instruction[1].parse().unwrap();

        for _i in 0..times {
            let from: &mut Vec<char> = &mut stacks[instruction[3].parse::<usize>().unwrap() - 1];
            let value = from.pop().unwrap();
            let to: &mut Vec<char> = &mut stacks[instruction[5].parse::<usize>().unwrap() - 1];
            to.push(value)
        }
    });
}

pub fn part_one(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");

    let mut stacks = parse_initial_state(&mut split);

    move_containers_9000(&mut split, &mut stacks);

    Some(read_result(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")))
    }
}
