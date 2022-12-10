use common::read_lines;

fn get_register_values(lines: &[String]) -> Vec<i32> {
    lines
        .iter()
        .fold((Vec::new(), 1), |(mut states, mut next), line| {
            if line == "noop" {
                states.push(next);
                return (states, next);
            }

            states.push(next);
            states.push(next);

            next += line
                .split(" ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            (states, next)
        })
        .0
}

fn solve_part_one(lines: &[String]) {
    let register_values = get_register_values(lines);
    println!(
        "{}",
        20 * register_values[19]
            + 60 * register_values[59]
            + 100 * register_values[99]
            + 140 * register_values[139]
            + 180 * register_values[179]
            + 220 * register_values[219]
    );
}

fn solve_part_two(lines: &[String]) {
    let register_values = get_register_values(lines);
    let mut cycle = 0;
    for _ in 0..6 {
        for i in 0..40 {
            print!(
                "{}",
                if (register_values[cycle] - i).abs() < 2 {
                    "#"
                } else {
                    "."
                }
            );

            cycle += 1;
        }
        println!();
    }
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/10/example").unwrap());
    solve_part_one(&read_lines("src/10/input").unwrap());

    println!("Part one:");
    solve_part_two(&read_lines("src/10/example").unwrap());
    solve_part_two(&read_lines("src/10/input").unwrap());
}
