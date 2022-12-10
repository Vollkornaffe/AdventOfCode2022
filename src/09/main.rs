use std::collections::HashSet;

use common::read_lines;

fn solve_part_one(lines: &[String]) {
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);

    for make_move in lines.iter().map(|line| {
        let mut split = line.split(" ");
        let direction = split.next().unwrap().to_string();
        let amount = split.next().unwrap().parse::<i32>().unwrap();
        move |(x, y): &mut (i32, i32)| match direction.as_str() {
            "R" => *x += amount,
            "L" => *x -= amount,
            "U" => *y += amount,
            "D" => *y -= amount,
            _ => unreachable!(),
        }
    }) {
        make_move(&mut head);
        while (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            tail.0 += (head.0 - tail.0).clamp(-1, 1);
            tail.1 += (head.1 - tail.1).clamp(-1, 1);
            visited.insert(tail);
        }
    }
    println!("{}", visited.len());
}

fn main() {
    solve_part_one(&read_lines("src/09/example").unwrap());
    solve_part_one(&read_lines("src/09/input").unwrap());
}
