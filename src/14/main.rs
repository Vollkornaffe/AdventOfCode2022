use std::{
    collections::HashSet,
    io::{self, Read},
};

use common::read_lines;

fn parse_rocks(lines: &[String]) -> HashSet<(i64, i64)> {
    lines.iter().fold(HashSet::new(), |set, line| {
        set.union(
            &line
                .split(" -> ")
                .map(|coords| {
                    let mut coords = coords.split(",");
                    (
                        coords.next().unwrap().parse::<i64>().unwrap(),
                        coords.next().unwrap().parse::<i64>().unwrap(),
                    )
                })
                .fold(
                    (HashSet::new(), None),
                    |(set, prev_coords), next_coords| match prev_coords {
                        None => (set, Some(next_coords)),
                        Some(prev_coords) => (
                            set.union(
                                &(prev_coords.0.min(next_coords.0)
                                    ..=prev_coords.0.max(next_coords.0))
                                    .map(|x| {
                                        (prev_coords.1.min(next_coords.1)
                                            ..=prev_coords.1.max(next_coords.1))
                                            .map(move |y| (x, y))
                                    })
                                    .flatten()
                                    .collect(),
                            )
                            .cloned()
                            .collect(),
                            Some(next_coords),
                        ),
                    },
                )
                .0,
        )
        .cloned()
        .collect()
    })
}

fn solve_part_one(lines: &[String]) {
    let rocks = parse_rocks(lines);

    let free_fall = rocks.iter().max_by_key(|pos| pos.1).unwrap().1;

    let mut sand = HashSet::new();

    'outer: loop {
        let mut current = (500, 0);

        loop {
            if current.1 > free_fall {
                break 'outer;
            }

            let blocked = |pos| rocks.contains(pos) || sand.contains(pos);
            let below = (current.0, current.1 + 1);
            let left = (current.0 - 1, current.1 + 1);
            let right = (current.0 + 1, current.1 + 1);

            if !blocked(&below) {
                current = below;
                continue;
            }
            if !blocked(&left) {
                current = left;
                continue;
            }
            if !blocked(&right) {
                current = right;
                continue;
            }

            break;
        }

        sand.insert(current);
    }

    println!("{}", sand.len());
}

fn solve_part_two(lines: &[String]) {
    let rocks = parse_rocks(lines);
    let floor = rocks.iter().max_by_key(|pos| pos.1).unwrap().1 + 2;

    let blocked_by_rock = |pos: &(i64, i64)| rocks.contains(pos) || pos.1 == floor;

    let mut sand = HashSet::new();

    loop {
        let mut current = (500, 0);

        loop {
            let blocked = |pos| blocked_by_rock(pos) || sand.contains(pos);

            let below = (current.0, current.1 + 1);
            let left = (current.0 - 1, current.1 + 1);
            let right = (current.0 + 1, current.1 + 1);

            if !blocked(&below) {
                current = below;
                continue;
            }
            if !blocked(&left) {
                current = left;
                continue;
            }
            if !blocked(&right) {
                current = right;
                continue;
            }

            break;
        }

        sand.insert(current);

        if current == (500, 0) {
            break;
        }
    }

    println!("{}", sand.len());
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/14/example").unwrap());
    solve_part_one(&read_lines("src/14/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/14/example").unwrap());
    solve_part_two(&read_lines("src/14/input").unwrap());
}
