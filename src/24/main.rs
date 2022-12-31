use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use common::read_lines;

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

fn solve(start: Vec2, end: Vec2, start_state: State) -> (usize, State) {
    let mut seen_states = HashSet::new();
    let mut active = VecDeque::new();

    active.push_back((0, start_state));

    loop {
        let (
            steps,
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

        assert!(setup
            .blizzards
            .iter()
            .find(|blizzard| blizzard.position == position)
            .is_none());

        if position == end {
            return (steps, State { position, setup });
        }

        setup.move_blizzards();

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
                position != start
                    && (position == end
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
                    State {
                        position,
                        setup: setup.clone(),
                    },
                )
            }),
        );
    }
}

fn solve_part_one(setup: Setup) {
    let (steps, _) = solve(
        setup.start(),
        setup.end(),
        State {
            position: setup.start(),
            setup,
        },
    );
    println!("{steps}");
}

fn main() {
    solve_part_one(Setup::new(&read_lines("src/24/example").unwrap()));
    solve_part_one(Setup::new(&read_lines("src/24/input").unwrap()));
}
