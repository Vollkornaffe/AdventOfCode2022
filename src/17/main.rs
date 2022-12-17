use std::{collections::HashSet, hash::Hash, iter::repeat};

use common::{read_lines, wait};

#[derive(Clone, Debug)]
struct Rock(pub Vec<(i32, i32)>);

impl Rock {
    //####
    fn minus() -> Self {
        Self(vec![(0, 0), (1, 0), (2, 0), (3, 0)])
    }

    //.#.
    //###
    //.#.
    fn plus() -> Self {
        Self(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)])
    }

    //..#
    //..#
    //..#
    //###
    fn angle() -> Self {
        Self(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (2, 2)])
    }

    //#
    //#
    //#
    //#
    fn vertical() -> Self {
        Self(vec![(0, 0), (0, 1), (0, 2), (0, 3)])
    }

    //##
    //##
    fn square() -> Self {
        Self(vec![(0, 0), (1, 0), (0, 1), (1, 1)])
    }

    fn try_move(&mut self, (mx, my): (i32, i32), map: &HashSet<(i32, i32)>) -> bool {
        let moved: Vec<(i32, i32)> = self.0.iter().map(|(x, y)| (x + mx, y + my)).collect();
        if moved
            .iter()
            .any(|&(x, y)| x < 0 || y < 0 || x > 6 || map.contains(&(x, y)))
        {
            false
        } else {
            self.0 = moved;
            true
        }
    }
}

fn setup(lines: &[String]) -> (Vec<i32>, Vec<Rock>) {
    (
        lines
            .first()
            .unwrap()
            .chars()
            .map(|c| match c {
                '>' => 1,
                '<' => -1,
                _ => unreachable!(),
            })
            .collect(),
        vec![
            Rock::minus(),
            Rock::plus(),
            Rock::angle(),
            Rock::vertical(),
            Rock::square(),
        ],
    )
}

fn get_highpoint(map: &HashSet<(i32, i32)>) -> i32 {
    map.iter().map(|(_, y)| *y + 1).max().unwrap_or(0)
}

fn drop_next<'a>(
    jets: &mut impl Iterator<Item = &'a i32>,
    rocks: &mut impl Iterator<Item = &'a Rock>,
    map: &mut HashSet<(i32, i32)>,
) {
    let highpoint = get_highpoint(&map);

    let mut rock = Rock(
        rocks
            .next()
            .unwrap()
            .0
            .iter()
            .map(|(x, y)| (x + 2, y + highpoint + 3))
            .collect(),
    );

    loop {
        rock.try_move((*jets.next().unwrap(), 0), &map);
        if rock.try_move((0, -1), &map) {
            continue;
        }
        map.extend(rock.0.iter());
        break;
    }
}

fn solve_part_one(lines: &[String]) {
    let (jets, rocks) = setup(lines);

    let mut jets = jets.iter().cycle();
    let mut rocks = rocks.iter().cycle();
    let mut map = HashSet::new();

    for _ in 0..2022 {
        drop_next(&mut jets, &mut rocks, &mut map);
    }

    println!("{}", get_highpoint(&map));
}

fn find_border(map: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut active = vec![(0, get_highpoint(map) + 1)];
    let mut visited = HashSet::new();
    let mut border = HashSet::new();

    while !active.is_empty() {
        let (x, y) = active.pop().unwrap();
        visited.insert((x, y));

        if map.contains(&(x, y)) {
            border.insert((x, y));
            continue;
        }

        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1)].into_iter() {
            if x < 0 || y < 0 || x > 6 || active.contains(&(x, y)) || visited.contains(&(x, y)) {
                continue;
            }
            active.push((x, y));
        }
    }

    border
}

fn debug(map: &HashSet<(i32, i32)>, border: &HashSet<(i32, i32)>) {
    for y in -100..1 {
        for x in 0..7 {
            if border.contains(&(x, -y)) {
                print!("@");
            } else if map.contains(&(x, -y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    wait();
}

fn solve_part_two(lines: &[String]) {
    let (jets, rocks) = setup(lines);

    let mut jets = jets.iter().cycle();
    let mut rocks = rocks.iter().cycle();
    let mut map = HashSet::new();

    let mut known_states = HashSet::new();

    for _ in 0..10000 {
        drop_next(&mut jets, &mut rocks, &mut map);

        let border = find_border(&map);
        let min = *border.iter().map(|(_, y)| y).min().unwrap();
        let mut key: Vec<(i32, i32)> = border.into_iter().map(|(x, y)| (x, y - min)).collect();
        key.sort();

        if known_states.insert(key) {
            println!("knew it!");
        }
    }
    println!("10000 : {}", known_states.len());
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/17/example").unwrap());
    solve_part_one(&read_lines("src/17/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/17/example").unwrap());
    solve_part_two(&read_lines("src/17/input").unwrap());
}
