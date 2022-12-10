pub struct Tree {
    height: u8,
    visible: bool,
}

pub fn visible_left(trees: &[Vec<Tree>], x: usize, y: usize) -> bool {
    let height = trees[x][y].height;
    for i in (0..x).rev() {
        if trees[i][y].height >= height {
            return false;
        }
    }
    true
}

pub fn visible_up(trees: &[Vec<Tree>], x: usize, y: usize) -> bool {
    let height = trees[x][y].height;
    for i in (0..y).rev() {
        if trees[x][i].height >= height {
            return false;
        }
    }
    true
}

pub fn visible_right(trees: &[Vec<Tree>], x: usize, y: usize, size: usize) -> bool {
    let height = trees[x][y].height;
    for line in trees.iter().take(size).skip(x + 1) {
        if line[y].height >= height {
            return false;
        }
    }
    true
}

pub fn visible_down(trees: &[Vec<Tree>], x: usize, y: usize, size: usize) -> bool {
    let height = trees[x][y].height;
    for i in (y + 1)..size {
        if trees[x][i].height >= height {
            return false;
        }
    }
    true
}

pub fn parse_and_process(input: &str, processor: &dyn Fn(&mut Vec<Vec<Tree>>, usize, usize, usize, usize) -> ()) -> Vec<Vec<Tree>> {
    let mut trees = input.lines().map(|line| {
        line.chars().map(|ch| {
            Tree { height: ch.to_digit(10).unwrap() as u8, visible: false }
        }).collect::<Vec<Tree>>()
    }).collect::<Vec<Vec<_>>>();

    let num_lines = trees.len();
    let num_trees = trees[0].len();

    for x in 0..num_lines {
        for y in 0..num_trees {
            processor(&mut trees, num_lines, num_trees, x, y)
        }
    }
    trees
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = parse_and_process(input, &is_tree_visible);
    let result = trees.iter().flatten().filter(|tree| { tree.visible }).count();
    Some(result as u32)
}

fn is_tree_visible(trees: &mut Vec<Vec<Tree>>, num_lines: usize, num_trees: usize, x: usize, y: usize) {
    if x == 0 || y == 0 || x == num_lines - 1 || y == num_trees - 1
        || visible_left(&trees, x, y)
        || visible_up(&trees, x, y)
        || visible_right(&trees, x, y, num_trees)
        || visible_down(&trees, x, y, num_lines)
    {
        trees[x][y].visible = true
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
