extern crate core;

use std::cell::RefCell;
use std::rc::Rc;

// TODO Works, but needs massive rewrite
struct Directory {
    name: String,
    size: u32,
    children: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new(name: String, size: u32) -> Directory {
        Directory {
            name,
            size,
            children: vec![],
            parent: None,
        }
    }

    pub fn find_dir(&self, min_size: u32, max_size: u32, root: Rc<RefCell<Directory>>) -> u32 {
        let mut found_size = max_size;
        let current = Rc::clone(&root);
        if current.borrow().size > min_size && current.borrow().size < max_size && !current.borrow().children.is_empty() {
            found_size = current.borrow().size;
        }
        for child in current.borrow().children.iter() {
            let new_found_size = current.borrow().find_dir(min_size, found_size, Rc::clone(child));
            if new_found_size < found_size {
                found_size = new_found_size
            }
        }
        found_size
    }

    pub fn horrible_parse(root: Rc<RefCell<Directory>>, input: &str) -> (u32, Rc<RefCell<Directory>>) {
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
                        if current_size < limit {
                            result += current_size;
                        }
                        let current_clone = Rc::clone(&current_dir);
                        current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                        current_dir.borrow_mut().size += current_size;
                    } else if name == "/" {
                        continue; //root already exists
                    } else {
                        let child = Rc::new(RefCell::new(Directory::new(name.to_string(), 0)));
                        current_dir.borrow_mut().children.push(Rc::clone(&child));
                        {
                            let mut mut_child = child.borrow_mut();
                            mut_child.parent = Some(Rc::clone(&current_dir));
                        }
                        current_dir = child
                    }
                } else if command == "ls" {
                    continue;
                } else {
                    panic!("Unexpected command!") //TODO Handle bete
                }
            } else if first == "dir" {
                continue;
            } else {
                let filename = parts.next().unwrap();
                let child = Rc::new(RefCell::new(Directory::new(filename.to_string(), first.parse::<u32>().unwrap())));
                let mut current_mut = current_dir.borrow_mut();
                current_mut.size += first.parse::<u32>().unwrap();
                current_mut.children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current_dir));
                }
            }
        }

        // ugly way to traverse back to root and count sizes on the way
        while current_dir.borrow().name != *"/" {
            let last_size = current_dir.borrow().size;
            let current_clone = Rc::clone(&current_dir);
            current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            let mut current_mut = current_dir.borrow_mut();
            current_mut.size += last_size;
        }
        (result, root)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = Rc::new(RefCell::new(Directory::new("/".to_string(), 0)));
    let (result, _) = Directory::horrible_parse(root, input);
    Some(result)
}


pub fn part_two(input: &str) -> Option<u32> {
    let root = Rc::new(RefCell::new(Directory::new("/".to_string(), 0)));
    {
        let _ = Directory::horrible_parse(root.clone(), input);
    }
    let disk_size = 70000000;
    let free_space = disk_size - root.borrow().size;
    let update_size = 30000000;
    let min_size = update_size - free_space;
    let result = root.borrow().find_dir(min_size, u32::MAX, root.clone());
    Some(result)
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
        assert_eq!(part_two(&input), Some(24933642));
    }
}
