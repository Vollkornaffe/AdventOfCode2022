use std::collections::{HashMap, HashSet};

use common::read_lines;

enum Direction {
    N,
    S,
    W,
    E,
}

fn parse_elves(lines: &[String]) -> HashSet<(isize, isize)> {
    lines
        .iter()
        .enumerate()
        .fold(Default::default(), move |set, (y, line)| {
            line.chars()
                .enumerate()
                .fold(set, move |mut set, (x, c)| match c {
                    '#' => {
                        set.insert((x as isize, y as isize));
                        set
                    }
                    _ => set,
                })
        })
}

fn solve_part_one(lines: &[String]) {
    let mut elves = parse_elves(lines);

    use Direction::*;
    let dirs = [N, S, W, E];
    let mut offset = 0;

    let get_proposal = |elves: &HashSet<(isize, isize)>, offset, (x, y)| {
        let mut dirs = dirs.iter().cycle().skip(offset);

        if [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .iter()
        .all(|p| !elves.contains(p))
        {
            return None;
        }

        for _ in 0..4 {
            let (propose, need_to_be_free) = match dirs.next().unwrap() {
                N => ((x, y - 1), [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]),
                S => ((x, y + 1), [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]),
                W => ((x - 1, y), [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]),
                E => ((x + 1, y), [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]),
            };

            if need_to_be_free.iter().all(|p| !elves.contains(p)) {
                return Some(propose);
            }
        }
        None
    };

    for _ in 0..10 {
        let mut targets = HashMap::new();
        for &elve in &elves {
            if let Some(proposal) = get_proposal(&elves, offset, elve) {
                targets.entry(proposal).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        elves = elves
            .iter()
            .map(|&elve| {
                let Some(proposal) = get_proposal(&elves, offset, elve) else {
                    return elve;
                };
                if targets[&proposal] == 1 {
                    proposal
                } else {
                    elve
                }
            })
            .collect();

        offset += 1;
        offset %= 4;
    }

    let mut free = 0;
    for x in *elves.iter().map(|(x, _)| x).min().unwrap()
        ..*elves.iter().map(|(x, _)| x).max().unwrap() + 1
    {
        for y in *elves.iter().map(|(_, y)| y).min().unwrap()
            ..*elves.iter().map(|(_, y)| y).max().unwrap() + 1
        {
            if !elves.contains(&(x, y)) {
                free += 1;
            }
        }
    }
    println!("{free}");
}

fn solve_part_two(lines: &[String]) {}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/23/example").unwrap());
    solve_part_one(&read_lines("src/23/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/23/example").unwrap());
    solve_part_two(&read_lines("src/23/input").unwrap());
}
