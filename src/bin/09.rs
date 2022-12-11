use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;

pub fn signum(x: i32) -> i32 {
    match x {
        1..=i32::MAX => { 1 }
        0 => { 0 }
        i32::MIN..=-1 => { -1 }
    }
}

struct Knot {
    x: i32,
    y: i32,
}

impl Clone for Knot {
    fn clone(&self) -> Self {
        Knot { x: self.x, y: self.y }
    }
}

struct LongRope {
    knots: Vec<RefCell<Knot>>,
}

impl LongRope {
    fn left(&mut self) -> (i32, i32) {
        self.knots[0].borrow_mut().x -= 1;
        self.move_tail()
    }

    fn right(&mut self) -> (i32, i32) {
        self.knots[0].borrow_mut().x += 1;
        self.move_tail()
    }

    fn up(&mut self) -> (i32, i32) {
        self.knots[0].borrow_mut().y -= 1;
        self.move_tail()
    }

    fn down(&mut self) -> (i32, i32) {
        self.knots[0].borrow_mut().y += 1;
        self.move_tail()
    }

    fn move_tail(&mut self) -> (i32, i32) {
        for i in 1..self.knots.len() {
            self.knots[i].borrow_mut().move_knot(self.knots[i - 1].borrow().deref());
        }
        (self.knots.last().unwrap().borrow().x, self.knots.last().unwrap().borrow().y)
    }
}

impl Knot {
    fn move_knot(&mut self, parent: &Knot) -> (i32, i32) {
        if (self.x - parent.x).abs() > 1 || (self.y - parent.y).abs() > 1 {
            self.y -= signum(self.y - parent.y);
            self.x -= signum(self.x - parent.x);
        }
        (self.x, self.y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    simulate_rope(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    simulate_rope(input, 10)
}

fn simulate_rope(input: &str, rope_length: usize) -> Option<u32> {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut rope = LongRope { knots: vec!(RefCell::new(Knot { x: 0, y: 0 }); rope_length) };
    input.lines().for_each(|command| {
        let mut iter = command.split(' ');
        let direction = iter.next().unwrap().chars().next().unwrap();
        let num_moves = iter.next().unwrap().parse::<i32>().unwrap();
        for _i in 0..num_moves {
            let tail_pos = match direction {
                'R' => { rope.right() }
                'L' => { rope.left() }
                'U' => { rope.up() }
                'D' => { rope.down() }
                _ => panic!("Unexpected input")
            };
            visited.insert(tail_pos);
        }
    });
    Some(visited.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
