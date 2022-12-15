use std::fmt::{Display, Formatter};
use crate::OrderResult::{No, Unknown, Yes};
use crate::Packet::{PList, PValue};

pub enum Packet {
    PValue(u32),
    PList(Vec<Packet>),
}

impl Packet {
    pub const fn is_value(&self) -> bool {
        matches!(*self, PValue(_))
    }
    pub const fn is_list(&self) -> bool {
        !self.is_value()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PValue(val) => { val.to_string() }
            PList(val) => { "[".to_owned() + &val.iter().map(|p| { p.to_string() }).collect::<Vec<String>>().join(",") + "]" }
        })
    }
}

pub enum OrderResult {
    Yes,
    No,
    Unknown,
}

pub fn correct_order(left: &Packet, right: &Packet) -> OrderResult {
    println!("Comparing {left} and {right}");
    let result = match left {
        PValue(left_value) => {
            match right {
                PValue(right_value) => {
                    match left_value {
                        l if l > right_value => { No }
                        l if l < right_value => { Yes }
                        _ => { Unknown }

                    }
                }
                PList(_) => {
                    correct_order(&PList(vec![PValue(*left_value)]), right)
                }
            }
        }
        PList(left_items) => {
            match right {
                PValue(right_value) => {
                    correct_order(left, &PList(vec![PValue(*right_value)]))
                }
                PList(right_items) => {
                    for (index, left_item) in left_items.iter().enumerate() {
                        if let Some(right_item) = right_items.get(index) {
                            let list_result = correct_order(left_item, right_item);
                            match list_result {
                                Yes => { return Yes; }
                                No => { return No; }
                                Unknown => { continue; }
                            }
                        } else {
                            return No;
                        }
                    }
                    return if right_items.len() > left_items.len() {
                        Yes
                    } else {
                        Unknown
                    };
                }
            }
        }
    };
    //println!("Result is {result}");
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let packets = input.split("\n\n");


    let mut index = 0;
    let result = packets.map(|pair| {
        let parsed_pair = pair.split('\n').map(|packet| {
            let mut char_stack = Vec::<char>::new();
            let mut parents = Vec::<Packet>::new();
            let mut current_num: Option<u32> = None;
            let mut current_plist: Option<Packet> = None;
            for c in packet.chars() {
                match c {
                    '[' => {
                        char_stack.push('[');
                        if let Some(current_plist) = current_plist {
                            parents.push(current_plist)
                        }
                        current_plist = Some(PList(Vec::new()));
                    }
                    ']' => {
                        if current_num.is_some() {
                            if let PList(items) = &mut current_plist.as_mut().unwrap() {
                                items.push(PValue(current_num.unwrap()))
                            }
                            current_num = None
                        }
                        let parent = parents.pop();
                        if let Some(mut parent) = parent {
                            if let PList(items) = &mut parent {
                                items.push(current_plist.unwrap());
                            }
                            current_plist = Some(parent);
                        }
                        char_stack.pop();
                    }
                    ',' => {
                        if let Some(value) = current_num {
                            if let PList(items) = &mut current_plist.as_mut().unwrap() {
                                items.push(PValue(value))
                            }
                        } // else previous is a list
                        current_num = None;
                    }
                    val => match current_num {
                        None => current_num = val.to_digit(10),
                        Some(n) => current_num = Some(n * 10 + val.to_digit(10).unwrap()),
                    },
                }
            }
            // println!("Parsed: {}", current_plist.as_ref().unwrap().unwrap());
            current_plist.unwrap()
        }).collect::<Vec<_>>();

        index += 1;
        let mut pairs = parsed_pair.iter();
        let order_result = correct_order(pairs.next().unwrap(), pairs.next().unwrap());
        match order_result {
            Yes => {
                // println!("Pair {index} IS in correct order");
                index
            }
            No => {
                // println!("Pair {index} is NOT in correct order");
                0
            }
            Unknown => {
                // println!("Pair {index} is in UNKNOWN order");
                0
            }
        }
    }).sum();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
