use std::collections::HashSet;

use common::read_lines;

fn get_neighbors(&[x, y, z]: &[isize; 3]) -> [[isize; 3]; 6] {
    [
        [x - 1, y, z],
        [x + 1, y, z],
        [x, y - 1, z],
        [x, y + 1, z],
        [x, y, z - 1],
        [x, y, z + 1],
    ]
}

fn solve_part_one(lines: &[String]) {
    let points: HashSet<[isize; 3]> = lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    println!(
        "{}",
        points
            .iter()
            .map(get_neighbors)
            .map(|ns| ns.iter().filter(|&n| !points.contains(n)).count() as isize)
            .sum::<isize>()
    );
}

fn main() {
    solve_part_one(&read_lines("src/18/example").unwrap());
    solve_part_one(&read_lines("src/18/input").unwrap());
}
