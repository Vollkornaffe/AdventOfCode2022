use std::collections::HashSet;

use common::read_lines;

fn solve_part_one(lines: &[String]) {
    let horizontals: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() + 1).collect())
        .collect();
    let n = horizontals.len(); // input is square
    let verticals = (0..n).fold(vec![Vec::new(); n], |mut verticals, i| {
        verticals
            .iter_mut()
            .enumerate()
            .for_each(|(j, vertical)| vertical.push(horizontals[i][j]));
        verticals
    });

    let mut visible = HashSet::new();

    for (y, row) in horizontals.iter().enumerate() {
        let mut height = 0;
        for (x, &tree) in row.iter().enumerate() {
            if tree > height {
                visible.insert((x, y));
                height = tree;
            }
        }
        let mut height = 0;
        for (x, &tree) in row.iter().enumerate().rev() {
            if tree > height {
                visible.insert((x, y));
                height = tree;
            }
        }
    }

    for (x, collumn) in verticals.iter().enumerate() {
        let mut height = 0;
        for (y, &tree) in collumn.iter().enumerate() {
            if tree > height {
                visible.insert((x, y));
                height = tree;
            }
        }
        let mut height = 0;
        for (y, &tree) in collumn.iter().enumerate().rev() {
            if tree > height {
                visible.insert((x, y));
                height = tree;
            }
        }
    }

    println!("{}", visible.len());
}

fn main() {
    solve_part_one(&read_lines("src/08/example").unwrap());
    solve_part_one(&read_lines("src/08/input").unwrap());
}
