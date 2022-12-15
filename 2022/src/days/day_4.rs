pub fn execute() {
    let file = include_str!("../../inputs/day_4_input");
    let mut counter_part_one = 0;
    let mut counter_part_two = 0;
    for line in file.lines() {
        let vec: Vec<i32> = line
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
