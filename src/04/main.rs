use common::read_lines;
use std::path::Path;

fn get_ranges<P: AsRef<Path>>(filename: P) -> Vec<((usize, usize), (usize, usize))> {
    read_lines(filename)
        .unwrap()
        .into_iter()
        .map(|line| {
            let mut splits = line.split(",");
            let mut splits_a = splits.next().unwrap().split("-");
            let mut splits_b = splits.next().unwrap().split("-");

            (
                (
                    splits_a.next().unwrap().parse().unwrap(),
                    splits_a.next().unwrap().parse().unwrap(),
                ),
                (
                    splits_b.next().unwrap().parse().unwrap(),
                    splits_b.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn solve_part_two<P: AsRef<Path>>(filename: P) -> usize {
    get_ranges(filename)
        .iter()
        .filter(|(a, b)| !(a.1 < b.0 || b.1 < a.0))
        .count()
}

fn solve_part_one<P: AsRef<Path>>(filename: P) -> usize {
    get_ranges(filename)
        .iter()
        .filter(|(a, b)| {
            ((a.0 < b.0 || a.0 == b.0) && (a.1 > b.1 || a.1 == b.1))
                || ((b.0 < a.0 || b.0 == a.0) && (b.1 > a.1 || b.1 == a.1))
        })
        .count()
}

fn main() {
    println!("Part one");
    println!("{}", solve_part_one("src/04/example"));
    println!("{}", solve_part_one("src/04/input"));
    println!("Part two");
    println!("{}", solve_part_two("src/04/example"));
    println!("{}", solve_part_two("src/04/input"));
}
