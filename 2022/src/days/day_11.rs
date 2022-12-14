use std::collections::HashMap;

use itertools::Itertools;

pub fn execute(rounds: usize, worry: bool) {
    let file = include_str!("../../inputs/day_11_input").replace("  ", "");
    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();
    for mut line in file.lines().chunks(7).into_iter() {
        let mut items: Vec<u128> = Vec::new();
        let mut throw: (u32, u32) = (0, 0);
        let id = line
            .next()
            .unwrap()
            .chars()
            .filter(|char| char.is_digit(10))
            .map(|digit| digit.to_digit(10).unwrap())
            .last()
            .unwrap();

        line.next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .into_iter()
            .for_each(|f| items.push(f.parse::<u128>().unwrap()));

        let operation = line.next().unwrap().split("old ").last().unwrap();

        let test = line
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();

        throw.0 = line
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        throw.1 = line
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        monkeys.insert(
            id,
            Monkey {
                id: id,
                items: items.clone(),
                operation: operation,
                test: test,
                throw: (throw.0, throw.1),
                count: 0,
            },
        );
    }
    let max = monkeys.iter().fold(1, |acc, e| acc * e.1.test);
    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            let cl = monkeys.clone();
            let monkey = cl.get(&(id as u32)).unwrap();
            let mut count = monkey.count;
            monkey.items.iter().for_each(|item| {
                count += 1;
                let operation = monkey.perform_operation(item, max, worry);
                let test_result = monkey.perform_test(&operation);
                if test_result {
                    let thrown_to = &mut monkeys.get(&monkey.throw.0).unwrap();
                    let items = &mut thrown_to.items.clone();
                    items.push(operation);
                    monkeys.insert(
                        thrown_to.id,
                        Monkey {
                            id: thrown_to.id,
                            items: items.clone(),
                            operation: thrown_to.operation,
                            test: thrown_to.test.clone(),
                            throw: thrown_to.throw,
                            count: thrown_to.count,
                        },
                    );
                } else {
                    let thrown_to = &mut monkeys.get(&monkey.throw.1).unwrap();
                    let mut items = thrown_to.items.clone();
                    items.push(operation);
                    monkeys.insert(
                        thrown_to.id,
                        Monkey {
                            id: thrown_to.id,
                            items: items.clone(),
                            operation: thrown_to.operation,
                            test: thrown_to.test.clone(),
                            throw: thrown_to.throw,
                            count: thrown_to.count,
                        },
                    );
                }
                monkeys.insert(
                    monkey.id,
                    Monkey {
                        id: monkey.id,
                        items: Vec::new(),
                        operation: monkey.operation,
                        test: monkey.test.clone(),
                        throw: monkey.throw,
                        count,
                    },
                );
            });
        }
    }
    let highest: u128 = monkeys
        .values()
        .sorted_by(|a, b| b.count.cmp(&a.count))
        .take(2)
        .fold(1, |acc, e| acc * e.count as u128);
    println!(
        "{} Total monkey business: {highest}",
        if worry { "Part 2:" } else { "Part 1:" }
    )
}

#[derive(Clone)]
struct Monkey<'a> {
    id: u32,
    items: Vec<u128>,
    operation: &'a str,
    test: u128,
    throw: (u32, u32),
    count: u32,
}

impl Monkey<'_> {
    fn perform_test(&self, item: &u128) -> bool {
        item % self.test == 0
    }

    fn perform_operation(&self, item: &u128, max: u128, worry: bool) -> u128 {
        let mut operation_string = self.operation.split_whitespace();
        let sign = operation_string.next().unwrap();
        let num = operation_string.next().unwrap();
        let result: u128;
        if num == "old" {
            result = item.clone();
        } else {
            result = num.parse::<u128>().unwrap()
        }
        match sign {
            "*" => {
                if worry {
                    return (item % max) * &result;
                } else {
                    return (item * &result) / 3;
                }
            }
            "+" => {
                if worry {
                    return (item % max) + &result;
                } else {
                    return (item + &result) / 3;
                }
            }
            &_ => panic!(),
        }
    }
}
