pub fn part_one(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");
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

    split.next().unwrap().lines().for_each(|line| {
        let instruction = line.split(' ').collect::<Vec<_>>();
        let times: u8 = instruction[1].parse().unwrap();

        for _i in 0..times {
            let from: &mut Vec<char> = &mut stacks[instruction[3].parse::<usize>().unwrap() - 1];
            let value = from.pop().unwrap();
            let to: &mut Vec<char> = &mut stacks[instruction[5].parse::<usize>().unwrap() - 1];
            to.push(value)
        }
    });

    Some(stacks.iter().map(|stack| {
        stack.last().unwrap_or(&' ')
    }).collect::<Vec<&char>>().into_iter().collect())
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
        assert_eq!(part_two(&input), None);
    }
}
