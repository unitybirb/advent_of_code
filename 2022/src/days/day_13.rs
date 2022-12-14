use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;

pub fn execute() {
    let file = include_str!("../../inputs/day_13_input");
    let mut correct = 0;
    let mut packets = vec![
        CmpList::List(vec![CmpList::List(vec![CmpList::Number(2)])]),
        CmpList::List(vec![CmpList::List(vec![CmpList::Number(6)])]),
    ];
    for mut line in file.lines().chunks(3).into_iter().enumerate() {
        let first_line = line.1.next().unwrap().replace("10", "a").parse::<CmpList>();
        let second_line = line.1.next().unwrap().replace("10", "a").parse::<CmpList>();
        if first_line <= second_line {
            correct += line.0 + 1;
        }
        packets.push(first_line.unwrap());
        packets.push(second_line.unwrap());
    }
    packets.sort();
    let found = packets.iter().positions(|f| {
        f == &CmpList::List(vec![CmpList::List(vec![CmpList::Number(2)])])
            || f == &CmpList::List(vec![CmpList::List(vec![CmpList::Number(6)])])
    });
    println!("Sum of correct packages: {correct}");
    println!("Decoder key: {}", found.fold(1, |acc, e| acc * (e + 1)))
}

#[derive(Debug, Eq, PartialEq)]
enum CmpList {
    Number(u32),
    List(Vec<Self>),
}

impl Ord for CmpList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for CmpList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => {
                let mut iter_a = a.iter();
                let mut iter_b = b.iter();
                loop {
                    match (iter_a.next(), iter_b.next()) {
                        (Some(a), Some(b)) => {
                            if let Some(order) = a.partial_cmp(b) {
                                if order != Ordering::Equal {
                                    return Some(order);
                                }
                            }
                        }
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (None, None) => return Some(Ordering::Equal),
                    }
                }
            }
            (Self::Number(a), Self::List(_)) => {
                Self::List(vec![Self::Number(*a)]).partial_cmp(other)
            }
            (Self::List(_), Self::Number(a)) => {
                self.partial_cmp(&Self::List(vec![Self::Number(*a)]))
            }
        }
    }
}

impl FromStr for CmpList {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut chars = line.chars().peekable();
        let mut packet = Self::List(Vec::new());
        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    let mut depth = 1;
                    let mut string = String::new();
                    while depth > 0 {
                        let c = chars.next().unwrap();
                        match c {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                        string.push(c);
                    }
                    if let Ok(p) = string[..string.len() - 1].parse() {
                        if let Self::List(list) = &mut packet {
                            list.push(p);
                        }
                    }
                }
                ',' => {}
                'a' => {
                    if let Self::List(list) = &mut packet {
                        list.push(Self::Number(10))
                    }
                }
                _ => {
                    if let Self::List(list) = &mut packet {
                        list.push(Self::Number(c.to_digit(10).unwrap()))
                    }
                }
            }
        }
        Ok(packet)
    }
}
