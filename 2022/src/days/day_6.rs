pub fn execute() {
    let file = include_str!("../../inputs/day_6_input");
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