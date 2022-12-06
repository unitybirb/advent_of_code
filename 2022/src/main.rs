use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    day_six()
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
        day_six_get_index(file, 4)
    );
    println!(
        "Part 2 - Found marker after {} characters",
        day_six_get_index(file, 14)
    )
}

fn day_six_get_index(file: &str, distinct_characters: usize) -> usize {
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

fn get_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Couldn't read input");
    BufReader::new(file)
}
