use std::path::Path;

use itertools::izip;

use common::read_lines;
fn solve_part_one(line: String) -> usize {
    let chars = line.chars().into_iter();
    4 + izip!(
        chars.clone(),
        chars.clone().skip(1),
        chars.clone().skip(2),
        chars.clone().skip(3),
    )
    .position(|(a, b, c, d)| a != b && a != c && a != d && b != c && b != d && c != d)
    .unwrap()
}

fn solve_part_two(line: String) -> usize {
    let chars = line.chars().into_iter();
    14 + izip!(
        chars.clone(),
        chars.clone().skip(1),
        chars.clone().skip(2),
        chars.clone().skip(3),
        chars.clone().skip(4),
        chars.clone().skip(5),
        chars.clone().skip(6),
        chars.clone().skip(7),
        chars.clone().skip(8),
        chars.clone().skip(9),
        chars.clone().skip(10),
        chars.clone().skip(11),
        chars.clone().skip(12),
        chars.clone().skip(13),
    )
    .position(
        |(c00, c01, c02, c03, c04, c05, c06, c07, c08, c09, c10, c11, c12, c13)| {
            c00 != c01
                && c00 != c02
                && c00 != c03
                && c00 != c04
                && c00 != c05
                && c00 != c06
                && c00 != c07
                && c00 != c08
                && c00 != c09
                && c00 != c10
                && c00 != c11
                && c00 != c12
                && c00 != c13
                && c01 != c02
                && c01 != c03
                && c01 != c04
                && c01 != c05
                && c01 != c06
                && c01 != c07
                && c01 != c08
                && c01 != c09
                && c01 != c10
                && c01 != c11
                && c01 != c12
                && c01 != c13
                && c02 != c03
                && c02 != c04
                && c02 != c05
                && c02 != c06
                && c02 != c07
                && c02 != c08
                && c02 != c09
                && c02 != c10
                && c02 != c11
                && c02 != c12
                && c02 != c13
                && c03 != c04
                && c03 != c05
                && c03 != c06
                && c03 != c07
                && c03 != c08
                && c03 != c09
                && c03 != c10
                && c03 != c11
                && c03 != c12
                && c03 != c13
                && c04 != c05
                && c04 != c06
                && c04 != c07
                && c04 != c08
                && c04 != c09
                && c04 != c10
                && c04 != c11
                && c04 != c12
                && c04 != c13
                && c05 != c06
                && c05 != c07
                && c05 != c08
                && c05 != c09
                && c05 != c10
                && c05 != c11
                && c05 != c12
                && c05 != c13
                && c06 != c07
                && c06 != c08
                && c06 != c09
                && c06 != c10
                && c06 != c11
                && c06 != c12
                && c06 != c13
                && c07 != c08
                && c07 != c09
                && c07 != c10
                && c07 != c11
                && c07 != c12
                && c07 != c13
                && c08 != c09
                && c08 != c10
                && c08 != c11
                && c08 != c12
                && c08 != c13
                && c09 != c10
                && c09 != c11
                && c09 != c12
                && c09 != c13
                && c10 != c11
                && c10 != c12
                && c10 != c13
                && c11 != c12
                && c11 != c13
                && c12 != c13
        },
    )
    .unwrap()
}

fn read_line<P: AsRef<Path>>(filename: P) -> String {
    read_lines(filename).unwrap().first().unwrap().to_string()
}

fn main() {
    println!("Part one");
    println!("{}", solve_part_one(read_line("src/06/example")));
    println!("{}", solve_part_one(read_line("src/06/input")));
    println!("Part two");
    println!("{}", solve_part_two(read_line("src/06/example")));
    println!("{}", solve_part_two(read_line("src/06/input")));
}
