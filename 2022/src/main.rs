use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    day_two("inputs/day_2_input")
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

fn get_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Couldn't read input");
    BufReader::new(file)
}
