pub fn execute() {
    let file = include_str!("../../inputs/day_5_input");
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
