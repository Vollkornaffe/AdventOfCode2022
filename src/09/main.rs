use std::collections::HashSet;

use common::read_lines;

fn solve(num_knots: usize, lines: &[String]) {
    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); num_knots];

    visited.insert(knots.last().cloned().unwrap());

    let follow = |head: &(i32, i32), tail: &mut (i32, i32)| -> bool {
        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            tail.0 += (head.0 - tail.0).clamp(-1, 1);
            tail.1 += (head.1 - tail.1).clamp(-1, 1);
            true
        } else {
            false
        }
    };

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
        make_move(&mut knots[0]);
        loop {
            let mut changed = false;
            let mut head = knots[0];
            for i in 1..knots.len() {
                if follow(&head, &mut knots[i]) {
                    changed = true;
                }
                head = knots[i];
            }
            visited.insert(knots.last().cloned().unwrap());
            if !changed {
                break;
            }
        }
    }

    for y in -50..50 {
        for x in -50..50 {
            if visited.contains(&(x, -y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }

    println!("{}", visited.len());
}

fn main() {
    println!("Part one:");
    solve(2, &read_lines("src/09/example").unwrap());
    solve(2, &read_lines("src/09/input").unwrap());

    println!("Part two:");
    solve(10, &read_lines("src/09/example2").unwrap());
    solve(10, &read_lines("src/09/input").unwrap());
}
