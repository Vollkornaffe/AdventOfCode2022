use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

use common::read_lines;

fn solve_part_one(lines: &[String]) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heights: Vec<Vec<u32>> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        00
                    }
                    'E' => {
                        end = (x, y);
                        25
                    }
                    'a' => 00,
                    'b' => 01,
                    'c' => 02,
                    'd' => 03,
                    'e' => 04,
                    'f' => 05,
                    'g' => 06,
                    'h' => 07,
                    'i' => 08,
                    'j' => 09,
                    'k' => 10,
                    'l' => 11,
                    'm' => 12,
                    'n' => 13,
                    'o' => 14,
                    'p' => 15,
                    'q' => 16,
                    'r' => 17,
                    's' => 18,
                    't' => 19,
                    'u' => 20,
                    'v' => 21,
                    'w' => 22,
                    'x' => 23,
                    'y' => 24,
                    'z' => 25,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // drop mut
    let start = start;
    let end = end;

    let size_x = heights[0].len();
    let size_y = heights.len();

    // ugly, bugly
    let options: Vec<Vec<Vec<(usize, usize)>>> = heights
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, height)| {
                    let mut options = Vec::new();
                    if x > 0 && heights[y][x - 1] < height + 2 {
                        options.push((x - 1, y));
                    }
                    if x < size_x - 1 && heights[y][x + 1] < height + 2 {
                        options.push((x + 1, y));
                    }
                    if y > 0 && heights[y - 1][x] < height + 2 {
                        options.push((x, y - 1));
                    }
                    if y < size_y - 1 && heights[y + 1][x] < height + 2 {
                        options.push((x, y + 1));
                    }
                    options
                })
                .collect()
        })
        .collect();

    let mut visited = HashSet::from([start]);
    let mut active = VecDeque::from([(start, 0)]);

    let debug_path = |visited: &HashSet<(usize, usize)>,
                      active: &VecDeque<((usize, usize), usize)>| {
        print!("{}[2J", 27 as char);
        for y in 0..size_y {
            for x in 0..size_x {
                if (x, y) == start {
                    print!("S");
                } else if (x, y) == end {
                    print!("E");
                } else if visited.contains(&(x, y)) {
                    print!("X");
                } else if active
                    .iter()
                    .find(|(active, _)| *active == (x, y))
                    .is_some()
                {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    };

    loop {
        println!();
        debug_path(&visited, &active);
        let mut tmp = String::new();
        stdin().read_line(&mut tmp).unwrap();

        let ((x, y), steps) = active.pop_front().unwrap();
        if (x, y) == end {
            println!("{}", steps);
            break;
        }

        visited.insert((x, y));
        let next_steps = options[y][x]
            .iter()
            .filter(|position| {
                !visited.contains(position)
                    && active
                        .iter()
                        .find(|(active, _)| active == *position)
                        .is_none()
            })
            .cloned()
            .map(|position| (position, steps + 1))
            .collect::<Vec<_>>();
        active.extend(next_steps.into_iter());
    }
}

fn main() {
    //solve_part_one(&read_lines("src/12/example").unwrap());
    solve_part_one(&read_lines("src/12/input").unwrap());
}
