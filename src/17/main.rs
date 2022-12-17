use std::{collections::HashSet, iter::repeat};

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

fn solve_part_one(lines: &[String]) {
    let jets: Vec<i32> = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => unreachable!(),
        })
        .collect();
    let rocks = vec![
        Rock::minus(),
        Rock::plus(),
        Rock::angle(),
        Rock::vertical(),
        Rock::square(),
    ];

    let mut jets = jets.iter().cycle();
    let mut rocks = rocks.iter().cycle();

    let mut map = HashSet::new();

    let get_highpoint =
        |map: &HashSet<(i32, i32)>| map.iter().map(|(_, y)| *y + 1).max().unwrap_or(0);

    for _ in 0..2022 {
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

    println!("{}", get_highpoint(&map));
}

fn main() {
    solve_part_one(&read_lines("src/17/example").unwrap());
    solve_part_one(&read_lines("src/17/input").unwrap());
}
