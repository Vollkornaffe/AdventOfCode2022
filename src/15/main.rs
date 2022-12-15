use std::collections::HashSet;

use common::read_lines;

fn dist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn solve_part_one(lines: &[String], check_y: i32) {
    let pairs = lines.iter().fold(HashSet::new(), |mut pairs, line| {
        let mut split = line.split(" ");
        let sensor: (i32, i32) = {
            let x = split.nth(2).unwrap();
            let y = split.nth(0).unwrap();
            (
                x[2..x.len() - 1].parse().unwrap(),
                y[2..y.len() - 1].parse().unwrap(),
            )
        };
        let beacon: (i32, i32) = {
            let x = split.nth(4).unwrap();
            let y = split.nth(0).unwrap();
            (x[2..x.len() - 1].parse().unwrap(), y[2..].parse().unwrap())
        };

        pairs.insert((sensor, beacon));
        pairs
    });

    let (min, max) = pairs.iter().fold(
        (
            (std::i32::MAX, std::i32::MAX),
            (std::i32::MIN, std::i32::MIN),
        ),
        |(min, max), (s, b)| {
            let max_dist = dist(s, b);
            (
                (min.0.min(s.0 - max_dist), min.1.min(s.1 - max_dist)),
                (max.0.max(s.0 + max_dist), max.1.max(s.1 + max_dist)),
            )
        },
    );

    println!(
        "{}",
        (min.0..=max.0)
            .filter(|x| {
                let p = &(*x, check_y);
                pairs.iter().all(|(_, b)| b != p)
                    && pairs.iter().any(|(s, b)| dist(s, p) <= dist(s, b))
            })
            .count()
    );
}

fn main() {
    solve_part_one(&read_lines("src/15/example").unwrap(), 10);
    solve_part_one(&read_lines("src/15/input").unwrap(), 2_000_000);
}
