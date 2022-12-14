pub fn execute() {
    let file = include_str!("../../inputs/day_10_input");
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
