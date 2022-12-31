use std::collections::HashSet;

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
                                'V' => {
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

fn solve_part_one(setup: Setup) {
    fn search(
        seen_states: &mut HashSet<State>,
        steps: usize,
        position: Vec2,
        mut setup: Setup,
    ) -> Option<usize> {
        //debug(steps, position, &setup);

        if position == setup.end() {
            println!("found one with {steps} steps");
            return Some(steps);
        }

        if !seen_states.insert(State {
            position,
            setup: setup.clone(),
        }) {
            return None;
        }

        setup.move_blizzards();

        let Vec2 { x, y } = position;
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
        .filter_map(|position| search(seen_states, steps + 1, position, setup.clone()))
        .min()
    }

    let mut seen_states = HashSet::new();

    println!("{:?}", search(&mut seen_states, 0, setup.start(), setup));
}

fn main() {
    solve_part_one(Setup::new(&read_lines("src/24/example").unwrap()));
    solve_part_one(Setup::new(&read_lines("src/24/input").unwrap()));
}
