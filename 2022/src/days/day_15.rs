use std::collections::HashSet;

pub fn execute() {
    const ROW: i32 = 2000000;
    const NUM_P2: i32 = 4000000;
    let file = include_str!("../../inputs/day_15_input").replace(":", ",");
    part_1(file.clone(),ROW);
    part_2(file, NUM_P2);
}

fn part_1(file: String, row: i32) {
    let mut coords: Vec<((i32, i32), (i32, i32), i32)> = Vec::new();
    for line in file.lines() {
        let filtered = line
            .chars()
            .filter(|f| f.is_digit(10) || *f == ',' || *f == '-')
            .collect::<String>();
        let mut split = filtered.split(|f| f == ',' || f == ':');
        let sensor = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        let beacon = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        let distance = distance(sensor, beacon);
        if (sensor.1 - distance..sensor.1 + distance).contains(&row) {
            coords.push((sensor, beacon, distance))
        };
    }
    let distances: HashSet<(i32, i32)> = coords
        .iter()
        .flat_map(|f| get_all_possible_beacons_for_distance(f.0, f.2, row))
        .collect();
    println!(
        "Part 1 - {:?} positions can't contain a beacon",
        distances.len() - 1
    );
}

fn part_2(file: String, num: i32) {
    let mut max = 0;
    let mut coords: Vec<((i32, i32), i32)> = Vec::new();
    for line in file.lines() {
        let filtered = line
            .chars()
            .filter(|f| f.is_digit(10) || *f == ',' || *f == '-')
            .collect::<String>();
        let mut split = filtered.split(|f| f == ',' || f == ':');
        let sensor = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        let beacon = (
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        );
        if (sensor.0) > max {
            max = sensor.0
        };
        let distance = distance(sensor, beacon);
        if !(sensor.0 + distance < 0 || sensor.0 - distance > num)
            && !(sensor.1 + distance < 0 || sensor.1 - distance > num)
        {
            coords.push((sensor, distance))
        }
    }
    let mut counter = 0;
    let bounds: HashSet<(i32, i32)> = coords
        .iter()
        .flat_map(|f| {
            println!("Getting {counter}th border");
            counter += 1;
            get_bounds(f.0, f.1)
        })
        .collect();
    let mut point: (i32, i32) = (-1, -1);
    for bound in bounds {
        let p1 = (bound.0 - 1, bound.1);
        let p2 = (bound.0 + 1, bound.1);
        if coords.iter().all(|f| {
            f.1 < distance(p1, f.0) && p1.0 >= 0 && p1.0 <= num && p1.1 >= 0 && p1.1 <= num
        }) {
            point = p1;
        } else if coords.iter().all(|f| {
            f.1 < distance(p2, f.0) && p2.0 >= 0 && p2.0 <= num && p2.1 >= 0 && p2.1 <= num
        }) {
            point = p2;
        }
    }
    println!("Point: {:?}", point);
    let calculation = (point.0 as i64) * (num as i64) + point.1 as i64;
    println!("Part2 - {:?}", calculation);
}

fn distance(same: (i32, i32), other: (i32, i32)) -> i32 {
    (same.0 - other.0).abs() + (same.1 - other.1).abs()
}
fn get_all_possible_beacons_for_distance(
    same: (i32, i32),
    distance: i32,
    row: i32,
) -> HashSet<(i32, i32)> {
    (same.0 - (distance - (same.1 - row).abs())..=same.0 + (distance - (same.1 - row).abs()))
        .map(|f| (f, row))
        .collect::<HashSet<(i32, i32)>>()
}

fn get_bounds(same: (i32, i32), distance: i32) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..=distance {
        set.insert((same.0 + i - distance, same.1 + i));
        set.insert((same.0 - i + distance, same.1 + i));
        set.insert((same.0 - i + distance, same.1 - i));
        set.insert((same.0 + i - distance, same.1 - i));
    }
    set
}
