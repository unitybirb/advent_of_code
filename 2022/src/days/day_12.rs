use itertools::Itertools;
use pathfinding::prelude::bfs;

pub fn execute() {
    let file = include_str!("../../inputs/day_12_input");
    let mut nodes: Vec<Pos> = Vec::new();
    let mut start: Pos = Pos(-1, -1, -1);
    let mut end: Pos = Pos(-1, -1, -1);
    let mut y = 0;

    for line in file.lines() {
        let mut x = 0;
        for char in line.chars() {
            let z = char.to_lowercase().last().unwrap() as i32 - 96;
            let position = Pos(x, y, z);
            if char == 'S' {
                start = Pos(position.0, position.1, 1);
                nodes.push(start.clone())
            } else if char == 'E' {
                end = Pos(position.0, position.1, 26);
                nodes.push(end.clone())
            } else {
                nodes.push(position)
            }
            x += 1;
        }
        y += 1;
    }
    let width = file.lines().next().unwrap().len();
    let result = bfs(
        &start,
        |p| p.successors(nodes.clone(), width),
        |p| p.0 == end.0 && p.1 == end.1,
    );
    let mut progress = 1;
    let start_vec = nodes.iter().filter(|p| p.2 == 1).collect_vec();
    /* TODO: improve part 2 */
    let result_part_two = start_vec
        .iter()
        .map(|f| {
            let result = bfs(
                *f,
                |p| p.successors(nodes.clone(), width),
                |p| p.0 == end.0 && p.1 == end.1,
            );
            println!(
                "{} paths out of {} finished, {:.2}% done",
                progress,
                start_vec.len(),
                (progress as f32 / start_vec.len() as f32 * 100.0)
            );
            progress += 1;
            match result {
                Some(pos) => pos.len() - 1,
                None => usize::MAX,
            }
        })
        .min()
        .unwrap();
    println!(
        "Part 1 - {} steps needed",
        result.expect("No path found").len() - 1
    );
    println!("Part 2 - {} steps needed", result_part_two);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Pos(i32, i32, i32);

impl Pos {
    fn successors(&self, nodes: Vec<Pos>, width: usize) -> Vec<Pos> {
        let position = nodes.iter().position(|p| p == self).unwrap();
        let mut possible_neighbors: Vec<Pos> = Vec::new();
        if position > 0 {
            possible_neighbors.push(nodes[position - 1])
        }
        if position > width {
            possible_neighbors.push(nodes[position - width])
        }
        if position < nodes.len() - width {
            possible_neighbors.push(nodes[position + width])
        }
        if position < nodes.len() - 1 {
            possible_neighbors.push(nodes[position + 1])
        }

        possible_neighbors
            .into_iter()
            .filter(|p| p.2 < self.2 + 2)
            .collect()
    }
}
