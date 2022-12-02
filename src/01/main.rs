use std::path::Path;

use common::read_lines;

fn get_sums<P: AsRef<Path>>(filename: P) -> Vec<usize> {
    read_lines(filename)
        .unwrap()
        .iter()
        .fold(vec![0], |mut elve_sums, next_item| {
            if next_item.is_empty() {
                elve_sums.push(0);
                elve_sums
            } else {
                *elve_sums.last_mut().unwrap() += next_item.parse::<usize>().unwrap();
                elve_sums
            }
        })
}

fn solve_part_one<P: AsRef<Path>>(filename: P) {
    println!("{}", get_sums(filename).iter().max().unwrap());
}

fn solve_part_two<P: AsRef<Path>>(filename: P) {
    let mut sums = get_sums(filename);
    sums.sort();
    sums.reverse();

    println!("{}", sums.iter().take(3).sum::<usize>());
}

fn main() {
    println!("Part one:");
    solve_part_one("src/01/example");
    solve_part_one("src/01/input");

    println!("Part two:");
    solve_part_two("src/01/example");
    solve_part_two("src/01/input");
}
