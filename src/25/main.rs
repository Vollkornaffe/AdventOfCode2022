use common::read_lines;

fn parse_snafu(s: &str) -> i64 {
    s.chars().rev().enumerate().fold(0, |n, (i, c)| match c {
        '=' => n - 5i64.pow(i as u32) * 2,
        '-' => n - 5i64.pow(i as u32),
        '0' => n,
        '1' => n + 5i64.pow(i as u32),
        '2' => n + 5i64.pow(i as u32) * 2,
        _ => unreachable!(),
    })
}

fn as_snafu(mut number: i64) -> String {
    let mut s = String::new();
    let mut carry: i64 = 0;
    while number != 0 {
        number += carry;
        carry = 0;
        let rest = number % 5;
        s = format!(
            "{}{s}",
            match rest {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => {
                    carry = 1;
                    "="
                }
                4 => {
                    carry = 1;
                    "-"
                }
                _ => unreachable!(),
            }
        );
        number /= 5;
    }
    s
}

fn solve_part_one(lines: &[String]) {
    println!(
        "{}",
        as_snafu(
            lines
                .iter()
                .map(|l| {
                    let n = parse_snafu(l.as_str());
                    n
                })
                .sum::<i64>()
        )
    );
}

fn main() {
    println!("Part one");
    solve_part_one(&read_lines("src/25/example").unwrap());
    solve_part_one(&read_lines("src/25/input").unwrap());
}
