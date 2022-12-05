use common::read_lines;
use std::path::Path;

fn get_crate_stacks(mut lines: Vec<String>) -> Vec<Vec<char>> {
    let n = lines.pop().unwrap().len() / 4 + 1; // remove the numbers line
    lines
        .iter()
        .rev()
        .fold(vec![Vec::new(); n], |stacks, line| {
            line.match_indices('[').fold(stacks, |mut stacks, (i, _)| {
                stacks[i / 4].push(line.chars().nth(i + 1).unwrap());
                stacks
            })
        })
}

fn solve<P: AsRef<Path>>(filename: P, one_or_two: bool) -> String {
    let lines = read_lines(filename).unwrap();
    let empty_pos = lines.iter().position(|s| s.is_empty()).unwrap();
    lines.as_slice()[empty_pos + 1..lines.len()]
        .iter()
        .fold(
            get_crate_stacks(lines.as_slice()[0..empty_pos].to_vec()),
            |mut stacks, line| {
                let split: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
                let count: usize = split[1].parse().unwrap();
                let source: usize = split[3].parse::<usize>().unwrap() - 1;
                let target: usize = split[5].parse::<usize>().unwrap() - 1;

                if one_or_two {
                    for _ in 0..count {
                        let tmp = stacks[source].pop().unwrap();
                        stacks[target].push(tmp);
                    }
                } else {
                    let range = stacks[source].len() - count..stacks[source].len();
                    let to_move: Vec<_> = stacks[source].drain(range).collect();
                    stacks[target].extend(to_move);
                }

                stacks
            },
        )
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn main() {
    println!("Part one");
    println!("{}", solve("src/05/example", true));
    println!("{}", solve("src/05/input", true));

    println!("Part two");
    println!("{}", solve("src/05/example", false));
    println!("{}", solve("src/05/input", false));
}
