use itertools::Itertools;
use std::{
    borrow::Borrow,
    collections::{btree_map::Values, HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    day_seven_no_tree()
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
    for content in cloned2.iter_mut() {
        for node in cloned.iter() {
            if content.1 == node.3.clone() {
                content.2 = content.2 + node.2;
            }
        }
    }
    let mut total_sum_cloned = 0;
    for ele in cloned2.clone() {
        total_sum_cloned = total_sum_cloned + ele.2
    }

    let clonedfiltered: Vec<&(u16, String, i32, String, bool)> = cloned2.iter().filter(|f| !f.4).collect();
    let mut total_sum_cloned_filtered:i64 = 0;
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
    let mut total_sum_after_map:i64 = 0;
    for ele in mapp.values().clone() {
        for ele2 in ele {
        total_sum_after_map = total_sum_after_map + ele2.2 as i64
        }
    }
    let mut mapped: HashMap<u16, Vec<(u16, String, i32, String)>> = HashMap::new();
    for i in 0..mapp.keys().len() + 2 {
        mapp.borrow().get(&(i as u16)).iter().for_each(|f| {
            let vv = f
                .iter()
                .map(|f| (f.0, f.1.clone(), f.2, f.3.clone()))
                .collect_vec();
            mapped.insert(i as u16, vv);
        });
    }

    for i in 1..mapped.keys().len() {
        let vecnames: Vec<String> = vec![];
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
    let mut adder:i64 = 0;
    for value in mapped.values() {
        for ele in value {
            if ele.2 <= 100000 {
                adder = adder + ele.2 as i64
            }
        }
    }
    mapped.iter().sorted().for_each(|f| {
        println!(" DEPTH {} LENGTH {}", f.0, f.1.len());
        f.1.iter().for_each(|x| print!(" {}, {}  PARENT {}", x.1, x.2,x.3));
        println!()
    });
    println!("Total folder size: {}\nTotal size of node list: {}\ntotal sum after first map: {}\nAfter clone: {}\nAfter filter: {}", adder, total_sum, total_sum_after_map, total_sum_cloned, total_sum_cloned_filtered);
}

fn get_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Couldn't read input");
    BufReader::new(file)
}
