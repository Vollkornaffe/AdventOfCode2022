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

fn parse_to_grid(lines: &[String]) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() + 1).collect())
        .collect()
}

fn solve_part_two(lines: &[String]) {
    let grid = parse_to_grid(lines);

    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let h = grid[y][x];

            score = score.max(
                {
                    let mut nx = x;
                    while nx != 0 {
                        nx -= 1;
                        if !(grid[y][nx] < h) {
                            break;
                        }
                    }
                    x - nx
                } * {
                    let mut nx = x;
                    while nx != grid.len() - 1 {
                        nx += 1;
                        if !(grid[y][nx] < h) {
                            break;
                        }
                    }
                    nx - x
                } * {
                    let mut ny = y;
                    while ny != 0 {
                        ny -= 1;
                        if !(grid[ny][x] < h) {
                            break;
                        }
                    }
                    y - ny
                } * {
                    let mut ny = y;
                    while ny != grid.len() - 1 {
                        ny += 1;
                        if !(grid[ny][x] < h) {
                            break;
                        }
                    }
                    ny - y
                },
            );
        }
    }

    println!("{}", score);
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/08/example").unwrap());
    solve_part_one(&read_lines("src/08/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/08/example").unwrap());
    solve_part_two(&read_lines("src/08/input").unwrap());
}
