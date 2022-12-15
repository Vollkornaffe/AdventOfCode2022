use std::collections::HashSet;

use common::read_lines;

fn dist(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse(lines: &[String]) -> Vec<((i64, i64), (i64, i64))> {
    lines.iter().fold(Vec::new(), |mut pairs, line| {
        let mut split = line.split(" ");
        let sensor: (i64, i64) = {
            let x = split.nth(2).unwrap();
            let y = split.nth(0).unwrap();
            (
                x[2..x.len() - 1].parse().unwrap(),
                y[2..y.len() - 1].parse().unwrap(),
            )
        };
        let beacon: (i64, i64) = {
            let x = split.nth(4).unwrap();
            let y = split.nth(0).unwrap();
            (x[2..x.len() - 1].parse().unwrap(), y[2..].parse().unwrap())
        };

        pairs.push((sensor, beacon));
        pairs
    })
}
fn check_coverage(pairs: &[((i64, i64), (i64, i64))], p: &(i64, i64)) -> bool {
    pairs.iter().all(|(_, b)| b != p) && pairs.iter().any(|(s, b)| dist(s, p) <= dist(s, b))
}

fn check_coverage_2(pairs: &[((i64, i64), (i64, i64))], p: &(i64, i64)) -> bool {
    pairs.iter().any(|(s, b)| dist(s, p) <= dist(s, b))
}

fn solve_part_one(lines: &[String], check_y: i64) {
    let pairs = parse(lines);
    let (min, max) = pairs.iter().fold(
        (
            (std::i64::MAX, std::i64::MAX),
            (std::i64::MIN, std::i64::MIN),
        ),
        |(min, max), (s, b)| {
            let max_dist = dist(s, b);
            (
                (min.0.min(s.0 - max_dist), min.1.min(s.1 - max_dist)),
                (max.0.max(s.0 + max_dist), max.1.max(s.1 + max_dist)),
            )
        },
    );
    println!("{:?}", (min, max));
    println!(
        "{}",
        (min.0..=max.0)
            .filter(|x| check_coverage(&pairs, &(*x, check_y)))
            .count()
    );
}

fn solve_part_two(lines: &[String], max_coord: i64) {
    let pairs = parse(lines);
    let check = |p: &(i64, i64)| {
        if p.0 < 0 || p.0 > max_coord || p.1 < 0 || p.1 > max_coord {
            return false;
        }
        if check_coverage_2(&pairs, &p) {
            return false;
        }

        println!("{:?}: {}", p, p.0 * 4_000_000 + p.1);
        true
    };
    for (s, b) in &pairs {
        let d = dist(s, b);

        let mut p = (s.0 - d - 1, s.1);

        for _ in 0..d + 1 {
            if check(&p) {
                return;
            }
            p.0 += 1;
            p.1 += 1;
        }

        assert!(p == (s.0, s.1 + d + 1));
        for _ in 0..d + 1 {
            if check(&p) {
                return;
            }
            p.0 += 1;
            p.1 -= 1;
        }

        assert!(p == (s.0 + d + 1, s.1));
        for _ in 0..d + 1 {
            if check(&p) {
                return;
            }
            p.0 -= 1;
            p.1 -= 1;
        }

        assert!(p == (s.0, s.1 - d - 1));
        for _ in 0..d + 1 {
            if check(&p) {
                return;
            }
            p.0 -= 1;
            p.1 += 1;
        }

        assert!(p == (s.0 - d - 1, s.1));
    }
    unreachable!()
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/15/example").unwrap(), 10);
    solve_part_one(&read_lines("src/15/input").unwrap(), 2_000_000);

    println!("Part two:");
    solve_part_two(&read_lines("src/15/example").unwrap(), 20);
    solve_part_two(&read_lines("src/15/input").unwrap(), 4_000_000);
}
