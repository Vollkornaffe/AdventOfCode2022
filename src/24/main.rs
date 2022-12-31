use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use common::{read_lines, wait};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Blizzard {
    position: Vec2,
    velocity: Vec2,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Setup {
    size: Vec2,
    blizzards: Vec<Blizzard>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    position: Vec2,
    setup: Setup,
}

impl Setup {
    fn new(lines: &[String]) -> Self {
        Self {
            size: Vec2 {
                x: lines[0].len() as isize,
                y: lines.len() as isize,
            },
            blizzards: lines
                .iter()
                .enumerate()
                .fold(Vec::new(), move |blizzards, (y, line)| {
                    line.chars()
                        .enumerate()
                        .fold(blizzards, move |mut blizzards, (x, c)| {
                            let position = Vec2 {
                                x: x as isize,
                                y: y as isize,
                            };
                            match c {
                                '^' => {
                                    blizzards.push(Blizzard {
                                        position,
                                        velocity: Vec2 { x: 0, y: -1 },
                                    });
                                }
                                'v' => {
                                    blizzards.push(Blizzard {
                                        position,
                                        velocity: Vec2 { x: 0, y: 1 },
                                    });
                                }
                                '<' => {
                                    blizzards.push(Blizzard {
                                        position,
                                        velocity: Vec2 { x: -1, y: 0 },
                                    });
                                }
                                '>' => {
                                    blizzards.push(Blizzard {
                                        position,
                                        velocity: Vec2 { x: 1, y: 0 },
                                    });
                                }
                                _ => {}
                            }
                            blizzards
                        })
                }),
        }
    }

    fn start(&self) -> Vec2 {
        Vec2 { x: 1, y: 0 }
    }

    fn end(&self) -> Vec2 {
        Vec2 {
            x: self.size.x - 2,
            y: self.size.y - 1,
        }
    }

    fn move_blizzards(&mut self) {
        self.blizzards.iter_mut().for_each(
            |Blizzard {
                 position: Vec2 { x: px, y: py },
                 velocity: Vec2 { x: vx, y: vy },
             }| {
                *px += *vx;
                *py += *vy;
                if *px == 0 {
                    *px = self.size.x - 2;
                }
                if *py == 0 {
                    *py = self.size.y - 2;
                }
                if *px == self.size.x - 1 {
                    *px = 1;
                }
                if *py == self.size.y - 1 {
                    *py = 1;
                }
            },
        );
    }
}

fn debug(steps: usize, position: Vec2, setup: &Setup) {
    println!("{steps}");
    for y in 0..setup.size.y {
        for x in 0..setup.size.x {
            if position.x == x && position.y == y {
                print!("O");
            } else if let Some(Blizzard { velocity, .. }) = setup
                .blizzards
                .iter()
                .find(|blizzard| blizzard.position == Vec2 { x, y })
            {
                print!(
                    "{}",
                    match velocity {
                        Vec2 { x: 0, y: -1 } => '^',
                        Vec2 { x: 0, y: 1 } => 'V',
                        Vec2 { x: -1, y: 0 } => '<',
                        Vec2 { x: 1, y: 0 } => '>',
                        _ => unreachable!(),
                    }
                );
            } else {
                print!(" ");
            }
        }
        println!();
    }
    wait();
}

fn solve_part_one(initial_setup: Setup) {
    let mut seen_states = HashSet::new();
    let mut active = VecDeque::new();

    active.push_back((
        0usize,
        Vec::new(),
        State {
            position: initial_setup.start(),
            setup: initial_setup.clone(),
        },
    ));

    while !active.is_empty() {
        let (
            steps,
            mut path,
            State {
                position,
                mut setup,
            },
        ) = active.pop_front().unwrap();

        if !seen_states.insert(State {
            position,
            setup: setup.clone(),
        }) {
            continue;
        }

        if position == setup.end() {
            println!("{steps}");

            let mut debug_setup = initial_setup;
            for position in path {
                debug(steps, position, &debug_setup);
                debug_setup.move_blizzards();
            }

            return;
        }

        setup.move_blizzards();

        path.push(position);

        let Vec2 { x, y } = position;
        active.extend(
            [
                Vec2 { x: x - 1, y },
                Vec2 { x: x + 1, y },
                Vec2 { x, y: y - 1 },
                Vec2 { x, y: y + 1 },
                Vec2 { x, y },
            ]
            .into_iter()
            .filter(|&position| {
                position != setup.start()
                    && (position == setup.end()
                        || (position.x > 0
                            && position.y > 0
                            && position.x < setup.size.x - 1
                            && position.y < setup.size.y - 1))
            })
            .filter(|position| {
                setup
                    .blizzards
                    .iter()
                    .find(|blizzard| blizzard.position == *position)
                    .is_none()
            })
            .map(|position| {
                (
                    steps + 1,
                    path.clone(),
                    State {
                        position,
                        setup: setup.clone(),
                    },
                )
            }),
        );
    }
}

fn main() {
    solve_part_one(Setup::new(&read_lines("src/24/example").unwrap()));
    solve_part_one(Setup::new(&read_lines("src/24/input").unwrap()));
}
