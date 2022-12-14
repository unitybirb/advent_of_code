use std::collections::HashSet;

pub fn execute(knots: usize) {
    let file = include_str!("../../inputs/day_9_input");
    let mut tail_positions: HashSet<Point> = HashSet::new();
    let mut knot_positions: Vec<Point> = vec![Point { x: 0, y: 0 }; knots];

    tail_positions.insert(Point { x: 0, y: 0 });

    for line in file.lines() {
        let mut instruction = line.split_whitespace();
        let direction = instruction.next().unwrap();
        let amount: i32 = instruction.next().unwrap().parse::<i32>().unwrap();
        for _ in 1..amount + 1 {
            let mut knot_positions_iter = knot_positions.iter();
            let mut current_head = knot_positions_iter.next().unwrap().move_head(direction);
            let mut rope: Vec<Point> = vec![current_head];
            for (index, knot) in knot_positions_iter.enumerate() {
                if knot.is_disconnected(current_head) {
                    let new_knot = knot.new_position(current_head);
                    rope.push(new_knot);
                    current_head = new_knot;
                    if index == knots - 2 {
                        tail_positions.insert(new_knot);
                    }
                } else {
                    rope.push(*knot);
                    current_head = *knot;
                }
            }
            knot_positions = rope;
        }
    }
    println!(
        "Number of positions visited with {} knots: {}",
        knots,
        tail_positions.len()
    );
}

#[derive(Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

trait GetPoints {
    fn is_disconnected(&self, head: Point) -> bool;
    fn new_position(&self, head: Point) -> Point;
    fn move_head(&self, direction: &str) -> Point;
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl GetPoints for Point {
    fn is_disconnected(&self, head: Point) -> bool {
        self.x.abs_diff(head.x) > 1 || self.y.abs_diff(head.y) > 1
    }

    fn new_position(&self, head: Point) -> Point {
        return Point {
            x: if self.x < head.x {
                self.x + 1
            } else if self.x > head.x {
                self.x - 1
            } else {
                self.x
            },
            y: if self.y < head.y {
                self.y + 1
            } else if self.y > head.y {
                self.y - 1
            } else {
                self.y
            },
        };
    }

    fn move_head(&self, direction: &str) -> Point {
        match direction {
            "U" => Point {
                x: self.x,
                y: self.y + 1,
            },
            "R" => Point {
                x: self.x + 1,
                y: self.y,
            },
            "L" => Point {
                x: self.x - 1,
                y: self.y,
            },
            "D" => Point {
                x: self.x,
                y: self.y - 1,
            },
            _ => {
                panic!()
            }
        }
    }
}