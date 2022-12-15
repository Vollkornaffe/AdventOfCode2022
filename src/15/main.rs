use std::collections::HashSet;

use common::read_lines;

fn solve_part_one(lines: &[String]) {
    let (covered, beacons) = lines.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut covered, mut beacons), line| {
            let mut split = line.split(" ");
            let sensor: (i32, i32) = {
                let x = split.nth(2).unwrap();
                let y = split.nth(0).unwrap();
                (
                    x[2..x.len() - 1].parse().unwrap(),
                    y[2..y.len() - 1].parse().unwrap(),
                )
            };
            let beacon: (i32, i32) = {
                let x = split.nth(4).unwrap();
                let y = split.nth(0).unwrap();
                (x[2..x.len() - 1].parse().unwrap(), y[2..].parse().unwrap())
            };

            beacons.insert(beacon);

            let max_dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            (sensor.0 - max_dist..sensor.0 + max_dist).for_each(|x| {
                (sensor.1 - max_dist..sensor.1 + max_dist).for_each(|y| {
                    if (x - sensor.0).abs() + (y - sensor.1).abs() <= max_dist {
                        covered.insert((x, y));
                    }
                })
            });
            (covered, beacons)
        },
    );

    println!(
        "{}",
        covered
            .iter()
            .filter(|(x, y)| *y == 10 && !beacons.contains(&(*x, *y)))
            .count()
    );
}

fn main() {
    solve_part_one(&read_lines("src/15/example").unwrap());
    solve_part_one(&read_lines("src/15/input").unwrap());
}
