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

// only works for my input..
fn jump_edge(from: &(isize, isize), orientation: isize) -> (isize, (isize, isize)) {
    let x = from.0 % 50;
    let y = from.1 % 50;
    match (from.0 / 50, from.1 / 50) {
        (0, 2) => match orientation {
            2 => (0, (50, 49 - y)),
            3 => (0, (50, 50 + x)),
            _ => unreachable!(),
        },
        (0, 3) => match orientation {
            0 => (3, (50 + y, 149)),
            1 => (1, (100 + x, 0)),
            2 => (1, (50 + y, 0)),
            _ => unreachable!(),
        },
        (1, 0) => match orientation {
            2 => (0, (0, 149 - y)),
            3 => (0, (0, 150 + x)),
            _ => unreachable!(),
        },
        (1, 1) => match orientation {
            0 => (3, (100 + y, 49)),
            2 => (1, (y, 100)),
            _ => unreachable!(),
        },
        (1, 2) => match orientation {
            0 => (2, (149, 49 - y)),
            1 => (2, (49, 150 + x)),
            _ => unreachable!(),
        },
        (2, 0) => match orientation {
            0 => (2, (99, 149 - y)),
            1 => (2, (99, 50 + x)),
            3 => (3, (x, 199)),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn solve_part_two(setup: Setup) {
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
                for _ in 0..n {
                    let dir = match orientation {
                        0 => (1, 0),
                        1 => (0, 1),
                        2 => (-1, 0),
                        3 => (0, -1),
                        _ => unreachable!(),
                    };

                    let (next_orientation, next_position) = {
                        let next_position = (position.0 + dir.0, position.1 + dir.1);
                        if setup.map.contains_key(&next_position) {
                            (orientation, next_position)
                        } else {
                            jump_edge(&position, orientation)
                        }
                    };
                    if setup.map[&next_position] {
                        position = next_position;
                        orientation = next_orientation;
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

    solve_part_two(parse_setup(&read_lines("src/22/input").unwrap()));
}
