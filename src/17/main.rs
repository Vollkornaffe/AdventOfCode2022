use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::repeat,
};

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
) -> u64 {
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

    let mut used_jets = 0;
    loop {
        used_jets += 1;
        rock.try_move((*jets.next().unwrap(), 0), &map);
        if rock.try_move((0, -1), &map) {
            continue;
        }
        map.extend(rock.0.iter());
        break;
    }
    used_jets
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

#[derive(Debug)]
struct LoopInfo {
    start: u64,
    length: u64,
    jet: u64,
    height: u64,
    increment: u64,
    border: Vec<(i32, i32)>,
}

fn solve_part_two(lines: &[String]) {
    let (jets_vector, rocks_vector) = setup(lines);

    let num_jets = jets_vector.len();

    let mut jets = jets_vector.iter().cycle();
    let mut rocks = rocks_vector.iter().cycle();
    let mut map = HashSet::new();

    let mut known_states = HashMap::new();

    let mut rock = 0;
    let mut jet = 0;

    let loop_info = loop {
        rock += 1;
        jet += drop_next(&mut jets, &mut rocks, &mut map);
        jet %= num_jets as u64;

        let border = {
            let border = find_border(&map);
            let min = *border.iter().map(|(_, y)| y).min().unwrap();
            let mut border: Vec<(i32, i32)> =
                border.into_iter().map(|(x, y)| (x, y - min)).collect();
            border.sort();
            border
        };

        let height = get_highpoint(&map) as u64;
        let key = (rock % 5, jet, border.clone());
        let value = (rock, height);
        if let Some((prev_rock, prev_height)) = known_states.insert(key, value) {
            break LoopInfo {
                start: prev_rock,
                length: rock - prev_rock,
                jet,
                height: prev_height,
                increment: height - prev_height,
                border,
            };
        }
    };

    let finish = 1_000_000_000_000u64;
    let repetitions = (finish - loop_info.start) / loop_info.length;

    let mut jets = jets_vector.iter().cycle().skip(loop_info.jet as usize);
    let mut rocks = rocks_vector.iter().cycle().skip(loop_info.start as usize);
    let mut map = loop_info.border.into_iter().collect();

    let init_height = get_highpoint(&map);

    let remaining = finish - loop_info.start - loop_info.length * repetitions;
    for _ in 0..remaining {
        drop_next(&mut jets, &mut rocks, &mut map);
    }

    let final_height = get_highpoint(&map);

    println!(
        "{}",
        loop_info.height + loop_info.increment * repetitions + final_height as u64
            - init_height as u64
    );
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/17/example").unwrap());
    solve_part_one(&read_lines("src/17/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/17/example").unwrap());
    solve_part_two(&read_lines("src/17/input").unwrap());
}
