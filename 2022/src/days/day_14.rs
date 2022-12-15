use std::cmp;

pub fn execute(part_2: bool) {
    let file = include_str!("../../inputs/day_14_input");

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
    if !part_2 {
        print_map(&area, multiplicator, min_x, part_2)
    };
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
        sand_x = if !part_2 { 500 - min_x } else { 1000 };
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

pub(crate) fn create_map(
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
                            area[(x + 500) + (2000 * coordinate.1)] = true;
                        }
                    }
                }
                if last.1 != coordinate.1 {
                    for y in cmp::min(last.1, coordinate.1)..=cmp::max(last.1, coordinate.1) {
                        if !part_2 {
                            area[act_x * y + (coordinate.0 - min_x)] = true;
                        } else {
                            area[2000 * y + (coordinate.0 + 500)] = true;
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
