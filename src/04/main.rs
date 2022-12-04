use common::read_lines;
use std::path::Path;

fn solve_part_one<P: AsRef<Path>>(filename: P) -> usize {
    read_lines(filename)
        .unwrap()
        .iter()
        .filter(|line| {
            let mut splits = line.split(",");
            let mut splits_a = splits.next().unwrap().split("-");
            let mut splits_b = splits.next().unwrap().split("-");

            let a: (usize, usize) = (
                splits_a.next().unwrap().parse().unwrap(),
                splits_a.next().unwrap().parse().unwrap(),
            );
            let b: (usize, usize) = (
                splits_b.next().unwrap().parse().unwrap(),
                splits_b.next().unwrap().parse().unwrap(),
            );

            ((a.0 < b.0 || a.0 == b.0) && (a.1 > b.1 || a.1 == b.1))
                || ((b.0 < a.0 || b.0 == a.0) && (b.1 > a.1 || b.1 == a.1))
        })
        .count()
}

fn main() {
    println!("Part one");
    println!("{}", solve_part_one("src/04/example"));
    println!("{}", solve_part_one("src/04/input"));
}
