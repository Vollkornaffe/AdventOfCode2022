use common::read_lines;
use std::path::Path;

fn solve(line: String, count: usize) -> usize {
    let chars: Vec<_> = line.chars().collect();
    count
        + chars
            .as_slice()
            .windows(count)
            .position(|window| {
                window
                    .iter()
                    .all(|a| window.iter().filter(|c| a == *c).count() == 1)
            })
            .unwrap()
}

fn read_line<P: AsRef<Path>>(filename: P) -> String {
    read_lines(filename).unwrap().first().unwrap().to_string()
}

fn main() {
    println!("Part one");
    println!("{}", solve(read_line("src/06/example"), 4));
    println!("{}", solve(read_line("src/06/input"), 4));
    println!("Part two");
    println!("{}", solve(read_line("src/06/example"), 14));
    println!("{}", solve(read_line("src/06/input"), 14));
}
