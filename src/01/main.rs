use std::path::Path;

use common::read_lines;

fn solve<P: AsRef<Path>>(filename: P) {
    let sums = read_lines(filename)
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
        });
    println!("{}", sums.iter().max().unwrap());
}

fn main() {
    solve("src/01/example");
    solve("src/01/input");
}
