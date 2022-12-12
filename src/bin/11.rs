use std::cell::RefCell;
use eval::Expr;

struct Monkey {
    items: RefCell<Vec<i64>>,
    operation: String,
    divisible_by: i64,
    true_destination: i32,
    false_destination: i32,
    index: i32,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkey_index = -1; // TODO ugly indexing
    let monkeys: Vec<Monkey> = input.split("\n\n").map(|monkey| {
        let mut lines = monkey.lines();
        let _ = lines.next();

        let starting_items = lines.next().unwrap().split(':').nth(1).unwrap().split(',').filter(|i| { !i.is_empty() });
        let starting_items: Vec<i64> = starting_items.map(|item| {
            item.trim().parse::<i64>().unwrap()
        }).collect();

        let x = lines.next().unwrap();
        let operation = x.split('=').nth(1).unwrap();
        let expr = operation.to_string();

        let divisible_by = lines.next().unwrap().split(' ').last().unwrap().parse::<i64>().unwrap();

        let true_destination = lines.next().unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();

        let false_destination = lines.next().unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();

        monkey_index += 1;

        Monkey { items: RefCell::new(starting_items), operation: expr, divisible_by, true_destination, false_destination, index: monkey_index }
    }).collect();
    monkeys
}

pub fn part_one(input: &str) -> Option<u32> {
    let monkeys = parse_input(input);

    let mut inspected = vec![0; monkeys.len()];

    for _i in 0..20 {
        for monkey in monkeys.iter() {
            while !monkey.items.borrow().is_empty() {
                let item:i64 = monkey.items.borrow_mut().drain(0..1).next().unwrap();
                inspected[monkey.index as usize] += 1;
                println!("Operation {}, old value {}", &monkey.operation, item);
                let new = serde_json::from_value::<i64>(Expr::new(&monkey.operation).value("old", item).exec().unwrap()).unwrap() / 3;
                if new % monkey.divisible_by == 0 {
                    monkeys[monkey.true_destination as usize].items.borrow_mut().push(new)
                } else {
                    monkeys[monkey.false_destination as usize].items.borrow_mut().push(new)
                }
            }
        }
    }

    inspected.sort();
    inspected.reverse();
    Some(inspected[0] * inspected[1])
}


pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
