use itertools::Itertools;

pub fn execute() {
    let file = include_str!("../../inputs/day_8_input");
    day_8_part_1(file);
    day_8_part_2(file)
}

fn day_8_part_1(file: &str) {
    let lines = file.lines();
    let mut forest: Vec<Vec<u32>> = vec![vec![]];
    forest.pop();
    let mut visible = 0;
    let mut iter2 = 0;
    let mut length = 0;
    let mut width = 0;
    for line in lines {
        width = 0;
        forest.push(
            line.chars()
                .into_iter()
                .map(|f| {
                    width += 1;
                    f.to_digit(10).unwrap()
                })
                .collect_vec(),
        );
        length += 1;
    }
    for (rowindex, row) in forest.iter().enumerate() {
        for (treeindex, tree) in row.iter().enumerate() {
            if treeindex == width {
                visible += 1;
                break;
            }
            iter2 = iter2 + 1;
            let row_clone = row.clone();
            let rows = row_clone.split_at(treeindex);
            let split_row_0 = rows.0;

            let split_row_1 = &rows.1[1..];

            if treeindex == 0
                || split_row_0.iter().all(|f| f < tree)
                || split_row_1.iter().all(|f| f < tree)
                || rowindex == 0
                || rowindex == length
                || treeindex == row.len() - 1
            {
                visible = visible + 1;
            } else {
                let mut column: Vec<u32> = vec![];
                let mut visible_bool = false;
                for iter in &forest {
                    column.push(iter[treeindex]);
                }
                let split_columns = column.split_at(rowindex);
                let split_col_0 = if rowindex > 0 && split_columns.0.len() == column.len() / 2 + 1 {
                    &split_columns.0[..rowindex - 1]
                } else {
                    split_columns.0
                };
                let split_col_1 = &split_columns.1[1..];
                if split_col_0.iter().all(|f| f < tree)
                    || split_col_1.iter().all(|f| f < tree)
                    || rowindex == length - 1
                {
                    visible_bool = true
                }
                if visible_bool {
                    visible = visible + 1;
                };
            }
        }
    }
    println!("Number of visible trees: {}", visible)
}

fn day_8_part_2(file: &str) {
    let mut scenic_vec: Vec<u32> = vec![];
    let lines = file.lines();
    let mut forest: Vec<Vec<u32>> = vec![vec![]];
    forest.pop();
    for line in lines {
        forest.push(
            line.chars()
                .into_iter()
                .map(|f| f.to_digit(10).unwrap())
                .collect_vec(),
        );
    }

    for (rowindex, row) in forest.iter().enumerate() {
        for (treeindex, tree) in row.iter().enumerate() {
            let mut scenic: u32 = 1;
            let row_clone = row.clone();
            let rows = row_clone.split_at(treeindex);
            let split_row_0 = rows.0;
            let split_row_1 = &rows.1[1..];
            let mut x = 0;

            for i in 0..split_row_0.len() {
                x += 1;
                if split_row_0[split_row_0.len() - 1 - i] >= *tree {
                    break;
                }
            }

            if x != 0 {
                scenic *= x;
            }
            x = 0;
            for ele in split_row_1 {
                x += 1;
                if ele >= tree {
                    break;
                }
            }
            if x != 0 {
                scenic *= x
            }

            let mut column: Vec<u32> = vec![];
            for iter in &forest {
                column.push(iter[treeindex]);
            }
            let split_columns = column.split_at(rowindex);
            let split_col_0 = if rowindex > 0 && split_columns.0.len() == column.len() / 2 + 1 {
                &split_columns.0[..rowindex]
            } else {
                split_columns.0
            };
            let split_col_1 = &split_columns.1[1..];
            let mut y = 0;
            for i in 0..split_col_0.len() {
                y += 1;
                if split_col_0[split_col_0.len() - 1 - i] >= *tree {
                    break;
                }
            }
            if y != 0 {
                scenic *= y;
            }
            y = 0;
            for ele in split_col_1 {
                y += 1;
                if ele >= tree {
                    break;
                }
            }
            if y != 0 {
                scenic *= y
            }

            scenic_vec.push(scenic);
        }
    }
    println!(
        "Number of visible trees: {}",
        scenic_vec.iter().max().unwrap()
    )
}