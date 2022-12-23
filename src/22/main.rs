use std::collections::HashMap;

use common::{read_lines, wait};

#[derive(Debug)]
enum Movement {
    TurnLeft,
    TurnRight,
    Forward(usize),
}

struct Setup {
    map: HashMap<(isize, isize), bool>,
    movements: Vec<Movement>,
}

fn parse_setup(lines: &[String]) -> Setup {
    let map =
        lines[0..lines.len() - 2]
            .iter()
            .enumerate()
            .fold(HashMap::new(), |map, (y, line)| {
                line.chars().enumerate().fold(map, |mut map, (x, c)| {
                    match c {
                        '.' => {
                            map.insert((x as isize, y as isize), true);
                        }
                        '#' => {
                            map.insert((x as isize, y as isize), false);
                        }
                        _ => {}
                    }
                    map
                })
            });

    let last_line = lines.last().unwrap();
    let mut last_number_start = None;
    let mut movements = Vec::new();

    for (i, c) in last_line.char_indices() {
        if c == 'L' || c == 'R' {
            if let Some(start) = last_number_start {
                movements.push(Movement::Forward(last_line[start..i].parse().unwrap()));
            }
            last_number_start = None;
        } else if last_number_start.is_none() {
            last_number_start = Some(i);
        }
        match c {
            'L' => movements.push(Movement::TurnLeft),
            'R' => movements.push(Movement::TurnRight),
            _ => {}
        }
    }

    if let Some(start) = last_number_start {
        movements.push(Movement::Forward(last_line[start..].parse().unwrap()));
    }

    Setup { map, movements }
}

fn wrap(
    map: &HashMap<(isize, isize), bool>,
    next: &(isize, isize),
    orientation: isize,
) -> (isize, isize) {
    match orientation {
        0 => (
            map.iter()
                .filter_map(|(&(x, y), _)| if y == next.1 { Some(x) } else { None })
                .min()
                .unwrap(),
            next.1,
        ),
        1 => (
            next.0,
            map.iter()
                .filter_map(|(&(x, y), _)| if x == next.0 { Some(y) } else { None })
                .min()
                .unwrap(),
        ),
        2 => (
            map.iter()
                .filter_map(|(&(x, y), _)| if y == next.1 { Some(x) } else { None })
                .max()
                .unwrap(),
            next.1,
        ),
        3 => (
            next.0,
            map.iter()
                .filter_map(|(&(x, y), _)| if x == next.0 { Some(y) } else { None })
                .max()
                .unwrap(),
        ),
        _ => unreachable!(),
    }
}

fn solve_part_one(setup: Setup) {
    let mut orientation = 0;
    let mut position = (
        *setup
            .map
            .iter()
            .filter(|&(&(_, y), &b)| b && y == 0)
            .map(|((x, _), _)| x)
            .min()
            .unwrap(),
        0,
    );

    for movement in setup.movements {
        match movement {
            Movement::TurnLeft => {
                orientation = (orientation + 3) % 4;
            }
            Movement::TurnRight => {
                orientation = (orientation + 1) % 4;
            }
            Movement::Forward(n) => {
                let dir = match orientation {
                    0 => (1, 0),
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    _ => unreachable!(),
                };
                for _ in 0..n {
                    let next = {
                        let next = (position.0 + dir.0, position.1 + dir.1);
                        if setup.map.contains_key(&next) {
                            next
                        } else {
                            wrap(&setup.map, &next, orientation)
                        }
                    };
                    if setup.map[&next] {
                        println!("{:?}", position);
                        for y in -20..20 {
                            for x in -20..20 {
                                let x = position.0 + x;
                                let y = position.1 + y;
                                if (x, y) == position {
                                    match orientation {
                                        0 => print!(">"),
                                        1 => print!("V"),
                                        2 => print!("<"),
                                        3 => print!("A"),
                                        _ => unreachable!(),
                                    }
                                } else if let Some(&free) = setup.map.get(&(x, y)) {
                                    if free {
                                        print!(".");
                                    } else {
                                        print!("#");
                                    }
                                } else {
                                    print!(" ");
                                }
                            }
                            println!();
                        }
                        wait();

                        position = next;
                    } else {
                        break;
                    }
                }
            }
        };
    }

    println!(
        "{}",
        1000 * (position.1 + 1) + 4 * (position.0 + 1) + orientation
    );
}

fn main() {
    solve_part_one(parse_setup(&read_lines("src/22/example").unwrap()));
    solve_part_one(parse_setup(&read_lines("src/22/input").unwrap()));
}
