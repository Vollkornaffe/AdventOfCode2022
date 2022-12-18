use std::collections::HashSet;

use common::read_lines;

fn get_neighbors(&[x, y, z]: &[isize; 3]) -> [[isize; 3]; 6] {
    [
        [x - 1, y, z],
        [x + 1, y, z],
        [x, y - 1, z],
        [x, y + 1, z],
        [x, y, z - 1],
        [x, y, z + 1],
    ]
}

fn parse_points(lines: &[String]) -> HashSet<[isize; 3]> {
    lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn solve_part_one(lines: &[String]) {
    let points = parse_points(lines);
    println!(
        "{}",
        points
            .iter()
            .map(get_neighbors)
            .map(|ns| ns.iter().filter(|&n| !points.contains(n)).count() as isize)
            .sum::<isize>()
    );
}

fn solve_part_two(lines: &[String]) {
    let points = parse_points(lines);

    let bounding_box = (
        [
            points.iter().map(|x| x[0]).min().unwrap(),
            points.iter().map(|x| x[1]).min().unwrap(),
            points.iter().map(|x| x[2]).min().unwrap(),
        ],
        [
            points.iter().map(|x| x[0]).max().unwrap(),
            points.iter().map(|x| x[1]).max().unwrap(),
            points.iter().map(|x| x[2]).max().unwrap(),
        ],
    );

    let bounding_box = (
        [
            bounding_box.0[0] - 1,
            bounding_box.0[1] - 1,
            bounding_box.0[2] - 1,
        ],
        [
            bounding_box.1[0] + 1,
            bounding_box.1[1] + 1,
            bounding_box.1[2] + 1,
        ],
    );

    let in_bounds = |&p: &[isize; 3]| {
        p[0] >= bounding_box.0[0]
            && p[1] >= bounding_box.0[1]
            && p[2] >= bounding_box.0[2]
            && p[0] <= bounding_box.1[0]
            && p[1] <= bounding_box.1[1]
            && p[2] <= bounding_box.1[2]
    };

    let mut visited = HashSet::from([bounding_box.0]);
    let mut active = vec![bounding_box.0];

    while let Some(p) = active.pop() {
        let new = get_neighbors(&p)
            .into_iter()
            .filter(in_bounds)
            .filter(|p| !visited.contains(p))
            .filter(|p| !points.contains(p))
            .collect::<Vec<_>>();
        active.extend(&new);
        visited.extend(new);
    }

    println!(
        "{}",
        points
            .iter()
            .map(get_neighbors)
            .map(|ns| {
                ns.iter()
                    .filter(|&n| !points.contains(n) && visited.contains(n))
                    .count() as isize
            })
            .sum::<isize>()
    );
}

fn main() {
    println!("part one");
    solve_part_one(&read_lines("src/18/example").unwrap());
    solve_part_one(&read_lines("src/18/input").unwrap());

    println!("part two");
    solve_part_two(&read_lines("src/18/example").unwrap());
    solve_part_two(&read_lines("src/18/input").unwrap());
}
