use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    day_one()
}

fn day_one() {
    let file = File::open("day_1_input").expect("Couldn't read input");
    let reader = BufReader::new(&file);
    let mut adder = 0;
    let mut calorie_vector: Vec<i32> = vec![];
    for line in reader.lines() {
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
