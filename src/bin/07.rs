extern crate core;

use std::cell::RefCell;
use std::rc::Rc;

struct Directory {
    name: String,
    size: u32,
    children: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new(name: String, size: u32) -> Directory {
        return Directory {
            name,
            size,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Directory>>) {
        self.children.push(new_node);
    }

    // pub fn print(&self) -> String {
    //     if let Some(name) = self.name {
    //         return name.to_string();
    //     } else {
    //         return String::from("[")
    //             + &self
    //             .children
    //             .iter()
    //             .map(|tn| tn.borrow().print())
    //             .collect::<Vec<String>>()
    //             .join(",")
    //             + "]";
    //     }
    // }
}


pub fn part_one(input: &str) -> Option<u32> {
    let root = Rc::new(RefCell::new(Directory::new("/".to_string(), 0)));
    let limit = 100000;
    let mut result: u32 = 0;
    let mut current_dir = Rc::clone(&root);
    for line in input.lines() {
        let mut parts = line.split(' ');
        let first = parts.next().unwrap();
        if first == "$" {
            let command = parts.next().unwrap();
            if command == "cd" {
                let name = parts.next().unwrap();
                if name == ".." {
                    let current_size = current_dir.borrow().size;
                    println!("{current_size}");
                    if current_size < limit {
                        result += current_size;
                    }
                    let current_clone = Rc::clone(&current_dir);
                    current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    current_dir.borrow_mut().size += current_size;
                } else {
                    if name == "/" {
                        // current_dir = Rc::new(RefCell::new(Directory::new(name.to_string(), 0)))
                        continue //root already exists
                    } else {
                        let child = Rc::new(RefCell::new(Directory::new(name.to_string(), 0)));
                        current_dir.borrow_mut().children.push(Rc::clone(&child));
                        {
                            let mut mut_child = child.borrow_mut();
                            mut_child.parent = Some(Rc::clone(&current_dir));
                        }
                        current_dir = child
                    }
                }

            } else if command == "ls" {
                continue;
            } else {
                panic!("Unexpected command!") //TODO Handle bete
            }
        } else if first == "dir" {
            continue
        } else {
            let filename = parts.next().unwrap();
            let child = Rc::new(RefCell::new(Directory::new(filename.to_string(), first.parse::<u32>().unwrap())));
            current_dir.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current_dir));
            }
            current_dir = child
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
