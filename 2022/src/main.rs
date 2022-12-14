use itertools::Itertools;
use pathfinding::prelude::bfs;

use std::{
    borrow::Borrow,
    cmp::{self, min_by, Ordering},
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    day_14(false);
    day_14(true)
}

fn day_one(reader: &BufReader<File>) {
    let mut adder = 0;
    let mut calorie_vector: Vec<i32> = vec![];
    for line in reader.buffer().as_ref().lines() {
        if !line.as_ref().unwrap().is_empty() {
            adder += line
                .expect("Couldn't read line")
                .parse::<i32>()
                .expect("Couldn't convert to i32");
        } else {
            calorie_vector.push(adder);
            adder = 0;
        }
    }
    calorie_vector.sort();
    let highest = &calorie_vector.pop().unwrap();
    let highest_three = highest + calorie_vector.pop().unwrap() + calorie_vector.pop().unwrap();
    println!(
        "Highest calories: {}\nThree highest calories added together: {}",
        highest, highest_three
    );
}

fn day_two(filename: &str) {
    day_two_part_one(get_file(filename));
    day_two_part_two(get_file(filename));
}

fn day_two_part_one(reader: BufReader<File>) {
    let possible_plays: HashMap<&str, i32> = HashMap::from([
        ("AX", 4),
        ("AY", 8),
        ("AZ", 3),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 7),
        ("CY", 2),
        ("CZ", 6),
    ]);
    let mut score = 0;
    for line in reader.lines() {
        let filtered: String = line.unwrap().split_whitespace().collect();
        score += *possible_plays.get(&filtered[..]).unwrap();
    }
    println!("Part 1 Total score: {}", score)
}

fn day_two_part_two(reader: BufReader<File>) {
    let alphabet = ['A', 'B', 'C'];
    let mut points = 0;
    for line in reader.lines() {
        let opponent_play = line.as_ref().unwrap().chars().next().unwrap();
        let index: i32 = alphabet
            .iter()
            .position(|&r| r == opponent_play)
            .unwrap()
            .try_into()
            .unwrap();
        let win_or_loss = line.as_ref().unwrap().chars().nth(2).unwrap();
        match win_or_loss {
            'X' => points += (index - 1).rem_euclid(3) + 1,
            'Y' => points += 3 + index + 1,
            'Z' => points += 6 + (index + 1) % 3 + 1,
            _ => {
                panic!("This should never happen, fix your code")
            }
        }
    }
    println!("Part 2 total score: {}", points)
}

fn day_three(filename: &str) {
    day_three_part_one(get_file(filename));
    day_three_part_two(get_file(filename))
}

fn day_three_part_one(reader: BufReader<File>) {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let mut priority = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let compartments = &line.split_at(line.len() / 2);
        let max = compartments
            .0
            .bytes()
            .into_iter()
            .filter(|x| compartments.1.bytes().any(|y| x.eq(&y)))
            .map(|x| alphabet.iter().position(|pos| x == *pos).unwrap())
            .max()
            .unwrap();
        priority += max + 1;
        println!("Total priority: {}", priority)
    }
}

fn day_three_part_two(reader: BufReader<File>) {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let mut priority = 0;
    let mut rucksacks: Vec<String> = Vec::new();
    let mut iterator = 0;
    for line in reader.lines() {
        rucksacks.push(line.unwrap());
        iterator += 1;
        if iterator == 3 {
            let rucksack_bytes: Vec<&[u8]> = rucksacks.iter().map(|x| x.as_bytes()).collect();
            let result: usize = rucksack_bytes[0]
                .iter()
                .filter(|x| rucksack_bytes[1].contains(x) && rucksack_bytes[2].contains(x))
                .map(|f| alphabet.iter().position(|pos| pos == f).unwrap())
                .max()
                .unwrap();
            iterator = 0;
            rucksacks = Vec::new();
            priority += result + 1
        }
    }
    println!("Final priority: {}", priority)
}

fn day_four(filename: &str) {
    let reader = get_file(filename);
    let mut counter_part_one = 0;
    let mut counter_part_two = 0;
    for line in reader.lines() {
        let vec: Vec<i32> = line
            .as_ref()
            .unwrap()
            .split(|delim| delim == ',' || delim == '-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if (vec[0] <= vec[3] && vec[1] >= vec[2]) || (vec[3] <= vec[0] && vec[2] >= vec[1]) {
            counter_part_two += 1
        }
        if (vec[0] <= vec[2] && vec[1] >= vec[3]) || (vec[2] <= vec[0] && vec[3] >= vec[1]) {
            counter_part_one += 1;
        }
    }
    println!(
        "Number of ranges where every element overlaps: {}",
        counter_part_one
    );
    println!(
        "Number of ranges where any element overlaps: {}",
        counter_part_two
    )
}

fn day_five() {
    let file = include_str!("../inputs/day_5_input");
    let filteredfile = file.replace("    ", ":").replace("   ", "");
    let lines = filteredfile.lines();
    let mut stacks: Vec<Vec<String>> = vec![Vec::with_capacity(9); 9];
    let mut stacks_part_two: Vec<Vec<String>> = vec![Vec::with_capacity(9); 9];
    for line in lines {
        if line.contains("[") {
            line.split(|delim| delim == ':' || delim == ' ')
                .collect::<Vec<&str>>()
                .iter()
                .enumerate()
                .for_each(|f| {
                    if !f.1.is_empty() {
                        stacks[f.0].insert(0, f.1.replace("[", "").replace("]", ""))
                    }
                });
        } else if line.is_empty() {
            stacks_part_two = stacks.clone();
        } else if line.starts_with("move") {
            let filtered: String = line
                .chars()
                .filter(|x| x.is_digit(10) || x.is_whitespace())
                .collect();
            let instructions: Vec<usize> = filtered
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().expect(x))
                .collect();
            let how_many = instructions[0];
            let from = instructions[1] - 1;
            let to = instructions[2] - 1;
            let mut temp_vec: Vec<String> = vec![];
            for _ in 0..how_many {
                let temp_stack = stacks[from].pop().unwrap();
                let temp_stack_part_2 = stacks_part_two[from].pop().unwrap();
                stacks[to].push(temp_stack);
                temp_vec.push(temp_stack_part_2);
            }
            temp_vec.reverse();
            temp_vec
                .iter()
                .for_each(|f| stacks_part_two[to].push(f.to_string()));
        }
    }
    println!("Part 1 - The following crates are on top of each stack: ");
    stacks.iter().for_each(|f| {
        print!("{}", f.to_owned().pop().unwrap());
    });
    println!("\nPart 2 - The following crates are on top of each stack: ");
    stacks_part_two.iter().for_each(|f| {
        print!("{}", f.to_owned().pop().unwrap());
    })
}

fn day_six() {
    let file = include_str!("../inputs/day_6_input");
    println!(
        "Part 1 - Found marker after {} characters",
        get_distinct_index(file, 4)
    );
    println!(
        "Part 2 - Found marker after {} characters",
        get_distinct_index(file, 14)
    )
}

fn get_distinct_index(file: &str, distinct_characters: usize) -> usize {
    let mut buf: Vec<char> = vec![' '; distinct_characters];
    let mut comparison = buf.clone();
    for (index, character) in file.chars().into_iter().enumerate() {
        buf[index % distinct_characters] = character;
        comparison[index % distinct_characters] = character;
        if index > 7 {
            buf.sort();
            buf.dedup();
            if buf.len() == distinct_characters {
                return index + 1;
            };
            buf = comparison.clone();
        }
    }
    /* Even easier solution:
    for i in 0..file.len() - distinct_characters {
        let mut slice: Vec<u8> = Vec::from(&file[i..i + distinct_characters]);
        slice.sort();
        slice.dedup();
        if slice.len() == distinct_characters {
            return i + distinct_characters;
        };
    }  */
    panic!("Couldn't find marker! Are you sure you picked the right file?");
}

fn day_seven_no_tree() {
    let file = include_str!("../inputs/day_7_input").replace("$ ", "");
    let mut previous_nodes: Vec<(u16, String, i32, String, bool)> = vec![];
    let mut node_list: Vec<(u16, String, i32, String, bool)> = vec![];
    let mut current_node: (u16, String, i32, String, bool) =
        (0, String::from(""), 0, String::from(""), false);
    let mut depth: u16 = 0;
    for line in file.lines().into_iter() {
        if line.eq("cd ..") {
            depth = depth - 1;
            current_node = previous_nodes.pop().unwrap();
        } else if line.eq("cd /") {
            let root = (0, String::from("/"), 0, String::from("ROOT"), false);
            node_list.push(root.clone());
            current_node = root.clone();
            previous_nodes.push(root);
        } else if line.starts_with("dir ") {
            let name = line.replace("dir ", "");
            let node = (depth + 1, name, 0, current_node.1.clone(), false);
            node_list.push(node);
        } else if line.starts_with("cd ") {
            let name = line.replace("cd ", "");
            let node: &(u16, String, i32, String, bool) = node_list
                .iter()
                .filter(|f| f.1 == name)
                .collect::<Vec<&(u16, String, i32, String, bool)>>()
                .first()
                .unwrap();
            previous_nodes.push(current_node);
            current_node = node.clone();
            depth = depth + 1;
        } else if line.starts_with(|s: char| s.is_numeric()) {
            let mut size = line.split_whitespace();
            let bits = size.next().unwrap().parse::<i32>().unwrap();
            let node = (
                depth + 1,
                String::from(size.clone().last().unwrap()),
                bits,
                current_node.clone().1,
                true,
            );
            node_list.push(node);
        }
    }
    let mut total_sum = 0;
    for ele in node_list.clone() {
        total_sum = total_sum + ele.2
    }
    let mut cloned2 = node_list.clone();
    let cloned = node_list.clone();
    for content in cloned.iter() {
        for node in cloned2.iter_mut() {
            if content.1 == node.3.clone() && content.0 == node.0 - 1 {
                node.2 = content.2 + node.2;
            }
        }
    }

    let mut total_sum_cloned = 0;
    for ele in cloned2.clone() {
        total_sum_cloned = total_sum_cloned + ele.2
    }

    let clonedfiltered: Vec<&(u16, String, i32, String, bool)> =
        cloned2.iter().filter(|f| !f.4).collect();
    let mut total_sum_cloned_filtered: i64 = 0;
    for ele in clonedfiltered.clone() {
        total_sum_cloned_filtered = total_sum_cloned_filtered + ele.2 as i64
    }

    let mut mapp: HashMap<u16, Vec<&(u16, String, i32, String, bool)>> = HashMap::new();
    let grouped = &clonedfiltered.iter().group_by(|f| f.0);
    for (key, it) in grouped {
        let borrowed = it.collect_vec();
        let entry = mapp.get(&key);
        let mut concat: Vec<&(u16, String, i32, String, bool)> = vec![];
        borrowed.iter().for_each(|f| {
            concat.push(f);
        });
        if entry.is_some() {
            entry.unwrap().iter().for_each(|x| {
                concat.push(x);
            })
        }
        mapp.insert(key, concat);
    }

    let mut total_sum_after_map: i64 = 0;
    for ele in mapp.values().clone() {
        for ele2 in ele {
            total_sum_after_map = total_sum_after_map + ele2.2 as i64
        }
    }

    let mut mapped: HashMap<u16, Vec<(u16, String, i32, String)>> = HashMap::new();
    for i in 0..mapp.keys().len() {
        mapp.borrow().get(&(i as u16)).iter().for_each(|f| {
            let vv = f
                .iter()
                .map(|f| (f.0, f.1.clone(), f.2, f.3.clone()))
                .collect_vec();
            mapped.insert(i as u16, vv);
        });
    }

    for i in 1..mapped.keys().len() {
        let index = (9 - i) as u16;
        let children = mapped.get(&index).unwrap();
        let mut parents = mapped.get(&(index - 1)).unwrap();
        let mut new_vector = vec![];
        for element in children {
            let name = element.3.clone();
            let item = parents
                .iter()
                .filter(|f| f.1 == name)
                .collect_vec()
                .first()
                .unwrap()
                .2
                + element.2;
            new_vector = parents
                .clone()
                .iter()
                .map(|f| {
                    if f.1 == name {
                        (f.0, f.1.clone(), item, f.3.clone())
                    } else {
                        f.clone()
                    }
                })
                .collect_vec();
            parents = &new_vector;
        }
        mapped.insert(index - 1, new_vector);
    }
    let mut adder: i64 = 0;
    for value in mapped.values() {
        for ele in value {
            if ele.2 <= 100000 {
                adder = adder + ele.2 as i64
            }
        }
    }
    mapped.iter().sorted().for_each(|f| {
        println!(" DEPTH {} LENGTH {}", f.0, f.1.len());
        f.1.iter()
            .for_each(|x| print!(" {}, {}  PARENT {}", x.1, x.2, x.3));
        println!()
    });
    let mut xz = 0;
    mapped.values().for_each(|f| xz = xz + f.len());
    println!("Total folder size: {}\nTotal size of node list: {}\ntotal sum after first map: {}\nAfter clone: {}\nAfter filter: {}", adder, total_sum, total_sum_after_map, total_sum_cloned, total_sum_cloned_filtered);
    println!(
        "Node list length {} \n{}cloned2 len: \n{}clonedfiltered len: \n{}map len: {}",
        node_list.len(),
        cloned2.len(),
        clonedfiltered.len(),
        xz,
        total_sum_cloned
    )
}

fn day_8() {
    day_8_part_1();
    day_8_part_2()
}

fn day_8_part_1() {
    let file = include_str!("../inputs/day_8_input");
    let lines = file.lines();
    let mut forest: Vec<Vec<u32>> = vec![vec![]];
    forest.pop();
    let mut visible = 0;
    let mut iter2 = 0;
    let mut length = 0;
    let mut width = 0;
    for line in lines {
        width = 0;
        forest.push(
            line.chars()
                .into_iter()
                .map(|f| {
                    width += 1;
                    f.to_digit(10).unwrap()
                })
                .collect_vec(),
        );
        length += 1;
    }
    for (rowindex, row) in forest.iter().enumerate() {
        for (treeindex, tree) in row.iter().enumerate() {
            if treeindex == width {
                visible += 1;
                break;
            }
            iter2 = iter2 + 1;
            let row_clone = row.clone();
            let rows = row_clone.split_at(treeindex);
            let split_row_0 = rows.0;

            let split_row_1 = &rows.1[1..];

            if treeindex == 0
                || split_row_0.iter().all(|f| f < tree)
                || split_row_1.iter().all(|f| f < tree)
                || rowindex == 0
                || rowindex == length
                || treeindex == row.len() - 1
            {
                visible = visible + 1;
            } else {
                let mut column: Vec<u32> = vec![];
                let mut visible_bool = false;
                for iter in &forest {
                    column.push(iter[treeindex]);
                }
                let split_columns = column.split_at(rowindex);
                let split_col_0 = if rowindex > 0 && split_columns.0.len() == column.len() / 2 + 1 {
                    &split_columns.0[..rowindex - 1]
                } else {
                    split_columns.0
                };
                let split_col_1 = &split_columns.1[1..];
                if split_col_0.iter().all(|f| f < tree)
                    || split_col_1.iter().all(|f| f < tree)
                    || rowindex == length - 1
                {
                    visible_bool = true
                }
                if visible_bool {
                    visible = visible + 1;
                };
            }
        }
    }
    println!("Number of visible trees: {}", visible)
}

fn day_8_part_2() {
    let file = include_str!("../inputs/day_8_input");
    let mut scenic_vec: Vec<u32> = vec![];
    let lines = file.lines();
    let mut forest: Vec<Vec<u32>> = vec![vec![]];
    forest.pop();
    for line in lines {
        forest.push(
            line.chars()
                .into_iter()
                .map(|f| f.to_digit(10).unwrap())
                .collect_vec(),
        );
    }

    for (rowindex, row) in forest.iter().enumerate() {
        for (treeindex, tree) in row.iter().enumerate() {
            let mut scenic: u32 = 1;
            let row_clone = row.clone();
            let rows = row_clone.split_at(treeindex);
            let split_row_0 = rows.0;
            let split_row_1 = &rows.1[1..];
            let mut x = 0;

            for i in 0..split_row_0.len() {
                x += 1;
                if split_row_0[split_row_0.len() - 1 - i] >= *tree {
                    break;
                }
            }

            if x != 0 {
                scenic *= x;
            }
            x = 0;
            for ele in split_row_1 {
                x += 1;
                if ele >= tree {
                    break;
                }
            }
            if x != 0 {
                scenic *= x
            }

            let mut column: Vec<u32> = vec![];
            for iter in &forest {
                column.push(iter[treeindex]);
            }
            let split_columns = column.split_at(rowindex);
            let split_col_0 = if rowindex > 0 && split_columns.0.len() == column.len() / 2 + 1 {
                &split_columns.0[..rowindex]
            } else {
                split_columns.0
            };
            let split_col_1 = &split_columns.1[1..];
            let mut y = 0;
            for i in 0..split_col_0.len() {
                y += 1;
                if split_col_0[split_col_0.len() - 1 - i] >= *tree {
                    break;
                }
            }
            if y != 0 {
                scenic *= y;
            }
            y = 0;
            for ele in split_col_1 {
                y += 1;
                if ele >= tree {
                    break;
                }
            }
            if y != 0 {
                scenic *= y
            }

            scenic_vec.push(scenic);
        }
    }
    println!(
        "Number of visible trees: {}",
        scenic_vec.iter().max().unwrap()
    )
}

fn day_9(knots: usize) {
    let file = include_str!("../inputs/day_9_input");
    let mut tail_positions: HashSet<Point> = HashSet::new();
    let mut knot_positions: Vec<Point> = vec![Point { x: 0, y: 0 }; knots];

    tail_positions.insert(Point { x: 0, y: 0 });

    for line in file.lines() {
        let mut instruction = line.split_whitespace();
        let direction = instruction.next().unwrap();
        let amount: i32 = instruction.next().unwrap().parse::<i32>().unwrap();
        for _ in 1..amount + 1 {
            let mut knot_positions_iter = knot_positions.iter();
            let mut current_head = knot_positions_iter.next().unwrap().move_head(direction);
            let mut rope: Vec<Point> = vec![current_head];
            for (index, knot) in knot_positions_iter.enumerate() {
                if knot.is_disconnected(current_head) {
                    let new_knot = knot.new_position(current_head);
                    rope.push(new_knot);
                    current_head = new_knot;
                    if index == knots - 2 {
                        tail_positions.insert(new_knot);
                    }
                } else {
                    rope.push(*knot);
                    current_head = *knot;
                }
            }
            knot_positions = rope;
        }
    }
    println!(
        "Number of positions visited with {} knots: {}",
        knots,
        tail_positions.len()
    );
}

#[derive(Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

trait GetPoints {
    fn is_disconnected(&self, head: Point) -> bool;
    fn new_position(&self, head: Point) -> Point;
    fn move_head(&self, direction: &str) -> Point;
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl GetPoints for Point {
    fn is_disconnected(&self, head: Point) -> bool {
        self.x.abs_diff(head.x) > 1 || self.y.abs_diff(head.y) > 1
    }

    fn new_position(&self, head: Point) -> Point {
        return Point {
            x: if self.x < head.x {
                self.x + 1
            } else if self.x > head.x {
                self.x - 1
            } else {
                self.x
            },
            y: if self.y < head.y {
                self.y + 1
            } else if self.y > head.y {
                self.y - 1
            } else {
                self.y
            },
        };
    }

    fn move_head(&self, direction: &str) -> Point {
        match direction {
            "U" => Point {
                x: self.x,
                y: self.y + 1,
            },
            "R" => Point {
                x: self.x + 1,
                y: self.y,
            },
            "L" => Point {
                x: self.x - 1,
                y: self.y,
            },
            "D" => Point {
                x: self.x,
                y: self.y - 1,
            },
            _ => {
                panic!()
            }
        }
    }
}

fn day_10() {
    let file = include_str!("../inputs/day_10_input");
    let mut x_register = 1;
    let mut cycle = 1;
    let mut signal_strengths: Vec<(i32, i32)> = Vec::new();
    let mut row: Vec<char> = Vec::with_capacity(40);
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        let mut input = line.split_whitespace();
        let instruction = input.next().unwrap();
        if instruction == "noop" {
            compute_cycles(
                &mut cycle,
                &mut signal_strengths,
                x_register,
                &mut row,
                &mut rows,
            );
        } else {
            let amount: i32 = input.next().unwrap().parse::<i32>().unwrap();
            for _ in 0..2 {
                compute_cycles(
                    &mut cycle,
                    &mut signal_strengths,
                    x_register,
                    &mut row,
                    &mut rows,
                );
            }
            x_register += amount
        }
    }

    let mut sum = 0;
    signal_strengths.iter().for_each(|f| sum += f.0 * f.1);
    println!("Sum of register values: {}", sum);
    rows.iter().for_each(|f| {
        f.iter().for_each(|x| {
            print!("{}", x);
        });
        println!();
    })
}

fn compute_cycles(
    cycle: &mut i32,
    signal_strengths: &mut Vec<(i32, i32)>,
    x_register: i32,
    row: &mut Vec<char>,
    rows: &mut Vec<Vec<char>>,
) {
    if [20, 60, 100, 140, 180, 220].contains(&*cycle) {
        signal_strengths.push((*cycle, x_register));
    }
    if (x_register..x_register + 3).contains(&(*cycle % 40)) {
        row.push('🦩');
    } else {
        row.push('🐴')
    }
    if *cycle % 40 == 0 {
        rows.push(row.clone());
        *row = Vec::new();
    }
    *cycle += 1;
}

fn day_11(rounds: usize, worry: bool) {
    let file = include_str!("../inputs/day_11_input").replace("  ", "");
    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();
    for mut line in file.lines().chunks(7).into_iter() {
        let mut items: Vec<u128> = Vec::new();
        let mut throw: (u32, u32) = (0, 0);
        let id = line
            .next()
            .unwrap()
            .chars()
            .filter(|char| char.is_digit(10))
            .map(|digit| digit.to_digit(10).unwrap())
            .last()
            .unwrap();

        line.next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .into_iter()
            .for_each(|f| items.push(f.parse::<u128>().unwrap()));

        let operation = line.next().unwrap().split("old ").last().unwrap();

        let test = line
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();

        throw.0 = line
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        throw.1 = line
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        monkeys.insert(
            id,
            Monkey {
                id: id,
                items: items.clone(),
                operation: operation,
                test: test,
                throw: (throw.0, throw.1),
                count: 0,
            },
        );
    }
    let max = monkeys.iter().fold(1, |acc, e| acc * e.1.test);
    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            let cl = monkeys.clone();
            let monkey = cl.get(&(id as u32)).unwrap();
            let mut count = monkey.count;
            monkey.items.iter().for_each(|item| {
                count += 1;
                let operation = monkey.perform_operation(item, max, worry);
                let test_result = monkey.perform_test(&operation);
                if test_result {
                    let thrown_to = &mut monkeys.get(&monkey.throw.0).unwrap();
                    let items = &mut thrown_to.items.clone();
                    items.push(operation);
                    monkeys.insert(
                        thrown_to.id,
                        Monkey {
                            id: thrown_to.id,
                            items: items.clone(),
                            operation: thrown_to.operation,
                            test: thrown_to.test.clone(),
                            throw: thrown_to.throw,
                            count: thrown_to.count,
                        },
                    );
                } else {
                    let thrown_to = &mut monkeys.get(&monkey.throw.1).unwrap();
                    let mut items = thrown_to.items.clone();
                    items.push(operation);
                    monkeys.insert(
                        thrown_to.id,
                        Monkey {
                            id: thrown_to.id,
                            items: items.clone(),
                            operation: thrown_to.operation,
                            test: thrown_to.test.clone(),
                            throw: thrown_to.throw,
                            count: thrown_to.count,
                        },
                    );
                }
                monkeys.insert(
                    monkey.id,
                    Monkey {
                        id: monkey.id,
                        items: Vec::new(),
                        operation: monkey.operation,
                        test: monkey.test.clone(),
                        throw: monkey.throw,
                        count,
                    },
                );
            });
        }
    }
    let highest: u128 = monkeys
        .values()
        .sorted_by(|a, b| b.count.cmp(&a.count))
        .take(2)
        .fold(1, |acc, e| acc * e.count as u128);
    println!(
        "{} Total monkey business: {highest}",
        if worry { "Part 2:" } else { "Part 1:" }
    )
}

#[derive(Clone)]
struct Monkey<'a> {
    id: u32,
    items: Vec<u128>,
    operation: &'a str,
    test: u128,
    throw: (u32, u32),
    count: u32,
}

impl Monkey<'_> {
    fn perform_test(&self, item: &u128) -> bool {
        item % self.test == 0
    }

    fn perform_operation(&self, item: &u128, max: u128, worry: bool) -> u128 {
        let mut operation_string = self.operation.split_whitespace();
        let sign = operation_string.next().unwrap();
        let num = operation_string.next().unwrap();
        let result: u128;
        if num == "old" {
            result = item.clone();
        } else {
            result = num.parse::<u128>().unwrap()
        }
        match sign {
            "*" => {
                if worry {
                    return (item % max) * &result;
                } else {
                    return (item * &result) / 3;
                }
            }
            "+" => {
                if worry {
                    return (item % max) + &result;
                } else {
                    return (item + &result) / 3;
                }
            }
            &_ => panic!(),
        }
    }
}

fn day_12() {
    let file = include_str!("../inputs/day_12_input");
    let mut nodes: Vec<Pos> = Vec::new();
    let mut start: Pos = Pos(-1, -1, -1);
    let mut end: Pos = Pos(-1, -1, -1);
    let mut y = 0;

    for line in file.lines() {
        let mut x = 0;
        for char in line.chars() {
            let z = char.to_lowercase().last().unwrap() as i32 - 96;
            let position = Pos(x, y, z);
            if char == 'S' {
                start = Pos(position.0, position.1, 1);
                nodes.push(start.clone())
            } else if char == 'E' {
                end = Pos(position.0, position.1, 26);
                nodes.push(end.clone())
            } else {
                nodes.push(position)
            }
            x += 1;
        }
        y += 1;
    }
    let width = file.lines().next().unwrap().len();
    let result = bfs(
        &start,
        |p| p.successors(nodes.clone(), width),
        |p| p.0 == end.0 && p.1 == end.1,
    );
    let mut progress = 1;
    let start_vec = nodes.iter().filter(|p| p.2 == 1).collect_vec();
    let result_part_two = start_vec
        .iter()
        .map(|f| {
            let result = bfs(
                *f,
                |p| p.successors(nodes.clone(), width),
                |p| p.0 == end.0 && p.1 == end.1,
            );
            println!(
                "{} paths out of {} finished, {:.2}% done",
                progress,
                start_vec.len(),
                (progress as f32 / start_vec.len() as f32 * 100.0)
            );
            progress += 1;
            match result {
                Some(pos) => pos.len() - 1,
                None => usize::MAX,
            }
        })
        .min()
        .unwrap();
    println!(
        "Part 1 - {} steps needed",
        result.expect("No path found").len() - 1
    );
    println!("Part 2 - {} steps needed", result_part_two);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Pos(i32, i32, i32);

impl Pos {
    fn successors(&self, nodes: Vec<Pos>, width: usize) -> Vec<Pos> {
        let position = nodes.iter().position(|p| p == self).unwrap();
        let mut possible_neighbors: Vec<Pos> = Vec::new();
        if position > 0 {
            possible_neighbors.push(nodes[position - 1])
        }
        if position > width {
            possible_neighbors.push(nodes[position - width])
        }
        if position < nodes.len() - width {
            possible_neighbors.push(nodes[position + width])
        }
        if position < nodes.len() - 1 {
            possible_neighbors.push(nodes[position + 1])
        }

        possible_neighbors
            .into_iter()
            .filter(|p| p.2 < self.2 + 2)
            .collect()
    }
}

fn day_13() {
    let file = include_str!("../inputs/day_13_input");
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
fn day_14(part_2: bool) {
    let file = include_str!("../inputs/day_14_input");

    let mut max_x: usize = 0;
    let mut min_x: usize = usize::MAX;
    let mut max_y: usize = 0;
    for line in file.replace(" ", "").lines() {
        let split = line.split("->");
        for it in split {
            let mut spl = it.split(",");
            let x = spl.next().unwrap().parse::<usize>().unwrap();
            let y = spl.next().unwrap().parse::<usize>().unwrap();
            if x > max_x {
                max_x = x
            };
            if y > max_y {
                max_y = y
            }
            if x < min_x {
                min_x = x
            }
        }
    }
    let act_x = max_x - min_x + 1;

    let mut area: Vec<bool> = if !part_2 {
        vec![false; act_x * (max_y + 3)]
    } else {
        vec![false; 2000 * (max_y + 3)]
    };
    create_map(file, &mut area, min_x, act_x, part_2, max_y);
    let multiplicator = if part_2 { 2000 } else { act_x };
    if !part_2{print_map(&area, multiplicator, min_x, part_2)};
    let counter = sand_fall(min_x, max_y, &mut area, multiplicator, part_2);
    println!("\nPart {} - {}", if part_2 { 2 } else { 1 }, counter);
}

fn sand_fall(min_x: usize, max_y: usize, area: &mut Vec<bool>, act_x: usize, part_2: bool) -> i32 {
    let mut previous: (usize, usize) = (usize::MAX, usize::MAX);
    let mut collision = true;
    let mut counter = 0;
    let mut temp_y = 0;
    let mut sand_x = if !part_2 { 500 - min_x } else { 1000 };
    let mut n = true;
    while n {
        while collision {
            if !area[sand_x + ((temp_y + 1) * act_x)] {
                temp_y += 1;
            } else {
                if !area[sand_x - 1 + ((temp_y + 1) * act_x)] {
                    temp_y += 1;
                    sand_x -= 1;
                } else {
                    if !area[sand_x + 1 + ((temp_y + 1) * act_x)] {
                        temp_y += 1;
                        sand_x += 1;
                    } else {
                        area[sand_x + (temp_y * act_x)] = true;
                        collision = false;
                        counter += 1;
                        if part_2 {
                            previous = (sand_x, temp_y);
                        }
                    }
                }
            }
            if !part_2 {
                if temp_y >= max_y || (sand_x == 0 && temp_y == max_y - 1) {
                    n = false;
                    break;
                }
            } else {
                if previous == (1000, 0) {
                    n = false;
                    break;
                }
            }
        }
        sand_x = if !part_2 {500 - min_x} else {1000};
        temp_y = 0;
        collision = true;
    }
    counter
}

/* doesn't work for part 2 lol */
fn print_map(area: &Vec<bool>, act_x: usize, min_x: usize, part_2: bool) {
    for (index, t) in area.iter().enumerate() {
        if index % act_x == 0 && index != 0 {
            println!()
        }
        if *t {
            print!("#")
        } else if (index == 500 - min_x && !part_2) || (index == 500 && part_2) {
            print!("V")
        } else if (800..1600).contains(&index) && part_2 {
            print!(".")
        } else if !part_2 {
            print!(".")
        }
    }
}

fn create_map(
    file: &str,
    area: &mut Vec<bool>,
    min_x: usize,
    act_x: usize,
    part_2: bool,
    max_y: usize,
) {
    for line in file.replace(" ", "").lines() {
        let mut split = line.split("->");
        let mut previous: Vec<(usize, usize)> = Vec::new();
        while let Some(num) = split.next() {
            let mut coordinates = num.split(",");
            let coordinate = (
                coordinates.next().unwrap().parse::<usize>().unwrap(),
                coordinates.next().unwrap().parse::<usize>().unwrap(),
            );
            if !previous.is_empty() {
                let last = previous.last().unwrap();
                if last.0 != coordinate.0 {
                    for x in cmp::min(last.0, coordinate.0)..=cmp::max(last.0, coordinate.0) {
                        if !part_2 {
                            area[(x - min_x) + (act_x * coordinate.1)] = true;
                        } else {
                            area[(x+500) + (2000 * coordinate.1)] = true;
                        }
                    }
                }
                if last.1 != coordinate.1 {
                    for y in cmp::min(last.1, coordinate.1)..=cmp::max(last.1, coordinate.1) {
                        if !part_2 {
                            area[act_x * y + (coordinate.0 - min_x)] = true;
                        } else {
                            area[2000 * y + (coordinate.0+500)] = true;
                        }
                    }
                }
            }
            previous.push(coordinate);
        }
    }
    if part_2 {
        for i in 0..2000 {
            area[i + (max_y + 2) * 2000] = true
        }
    }
}

fn get_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Couldn't read input");

    BufReader::new(file)
}
