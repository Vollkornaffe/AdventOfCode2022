use common::read_lines;

fn solve_part_one(lines: &[String]) {
    let mut mix = lines
        .iter()
        .enumerate()
        .map(|(idx, s)| (idx, s.parse::<isize>().unwrap()))
        .collect::<Vec<_>>();
    let n = mix.len();

    for idx in 0..n {
        let mut i = mix.iter().position(|e| e.0 == idx).unwrap();
        let mut dir = mix[i].1;
        while dir != 0 {
            let next = if dir < 0 {
                if i == 0 {
                    n - 1
                } else {
                    i - 1
                }
            } else {
                if i == n - 1 {
                    0
                } else {
                    i + 1
                }
            };
            mix.swap(i, next);
            i = next;
            dir -= dir.clamp(-1, 1);
        }
    }

    let zero = mix.iter().position(|e| e.1 == 0).unwrap();
    println!(
        "{}",
        mix[(zero + 1000) % n].1 + mix[(zero + 2000) % n].1 + mix[(zero + 3000) % n].1
    );
}

fn main() {
    solve_part_one(&read_lines("src/20/example").unwrap());
    solve_part_one(&read_lines("src/20/input").unwrap());
}
