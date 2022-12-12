use std::collections::{HashSet, VecDeque};

use common::read_lines;

struct Setup {
    start: (usize, usize),
    end: (usize, usize),
    heights: Vec<Vec<u32>>,
    _size_x: usize,
    _size_y: usize,
    options: Vec<Vec<Vec<(usize, usize)>>>,
}

impl Setup {
    fn new(lines: &[String]) -> Self {
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
        Self {
            start,
            end,
            heights,
            _size_x: size_x,
            _size_y: size_y,
            options,
        }
    }

    fn find_path_len(&self) -> Option<usize> {
        let mut visited = HashSet::from([self.start]);
        let mut active = VecDeque::from([(self.start, 0)]);

        loop {
            let ((x, y), steps) = match active.pop_front() {
                None => return None,
                Some(next) => next,
            };
            if (x, y) == self.end {
                return Some(steps);
            }

            visited.insert((x, y));
            let next_steps = self.options[y][x]
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
}

#[allow(dead_code)]
fn debug_path(
    start: (usize, usize),
    end: (usize, usize),
    size_x: usize,
    size_y: usize,
    visited: &HashSet<(usize, usize)>,
    active: &VecDeque<((usize, usize), usize)>,
) {
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
}

fn solve_part_one(setup: Setup) {
    println!("{:?}", setup.find_path_len());
}

fn solve_part_two(mut setup: Setup) {
    let possible_starts: Vec<(usize, usize)> = setup
        .heights
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, h)| if *h == 0 { Some((x, y)) } else { None })
        })
        .flatten()
        .collect();

    let mut shortest = std::usize::MAX;
    for start in possible_starts {
        setup.start = start;
        if let Some(len) = setup.find_path_len() {
            shortest = shortest.min(len);
        }
    }

    println!("{}", shortest);
}

fn main() {
    println!("Part one:");
    solve_part_one(Setup::new(&read_lines("src/12/example").unwrap()));
    solve_part_one(Setup::new(&read_lines("src/12/input").unwrap()));

    println!("Part two:");
    solve_part_two(Setup::new(&read_lines("src/12/example").unwrap()));
    solve_part_two(Setup::new(&read_lines("src/12/input").unwrap()));
}
