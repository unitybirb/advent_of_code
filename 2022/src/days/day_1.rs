pub fn execute() {
    let file = include_str!("../../inputs/day_1_input");
    let mut adder = 0;
    let mut calorie_vector: Vec<i32> = vec![];
    for line in file.lines() {
        if !line.is_empty() {
            adder += line.parse::<i32>().expect("Couldn't convert to i32");
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
