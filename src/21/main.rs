use std::collections::HashMap;

use common::read_lines;

enum Job {
    A,
    S,
    M,
    D,
}

fn get_maps(
    lines: &[String],
) -> (
    HashMap<&str, isize>,
    HashMap<&str, Vec<&str>>,
    HashMap<&str, Job>,
    HashMap<&str, (&str, &str)>,
) {
    lines.iter().fold(
        Default::default(),
        |(mut value_map, mut dep_map, mut job_map, mut in_map), line| {
            let split = line.split(" ").collect::<Vec<_>>();

            let name = &split[0][..split[0].len() - 1];
            if split.len() == 2 {
                value_map.insert(name, split[1].parse().unwrap());
            } else {
                if let Some(deps) = dep_map.get_mut(split[1]) {
                    deps.push(name);
                } else {
                    dep_map.insert(split[1], vec![name]);
                }
                if let Some(deps) = dep_map.get_mut(split[3]) {
                    deps.push(name);
                } else {
                    dep_map.insert(split[3], vec![name]);
                }

                job_map.insert(
                    name,
                    match split[2] {
                        "+" => Job::A,
                        "-" => Job::S,
                        "*" => Job::M,
                        "/" => Job::D,
                        _ => unreachable!(),
                    },
                );

                in_map.insert(name, (split[1], split[3]));
            }

            (value_map, dep_map, job_map, in_map)
        },
    )
}

fn calc<'a>(
    value_map: &mut HashMap<&'a str, isize>,
    dep_map: &HashMap<&str, Vec<&'a str>>,
    job_map: &HashMap<&str, Job>,
    in_map: &HashMap<&str, (&str, &str)>,
    mut active: Vec<&'a str>,
) {
    while !active.is_empty() {
        let current = active.pop().unwrap();
        for &dep in dep_map.get(current).or(Some(&Vec::new())).unwrap() {
            let (a, b) = in_map[dep];
            if let (Some(a), Some(b)) = (value_map.get(a), value_map.get(b)) {
                active.push(dep);
                match job_map[dep] {
                    Job::A => {
                        value_map.insert(dep, a + b);
                    }
                    Job::S => {
                        value_map.insert(dep, a - b);
                    }
                    Job::M => {
                        value_map.insert(dep, a * b);
                    }
                    Job::D => {
                        value_map.insert(dep, a / b);
                    }
                }
            }
        }
    }
}

fn solve_part_one(lines: &[String]) {
    let (mut value_map, dep_map, job_map, in_map) = get_maps(lines);

    let active = value_map.keys().cloned().collect();
    calc(&mut value_map, &dep_map, &job_map, &in_map, active);

    println!("{}", value_map["root"]);
}

fn solve_part_two(lines: &[String]) {
    let (mut value_map, dep_map, job_map, in_map) = get_maps(lines);

    assert!(dep_map.values().all(|dep| dep.len() == 1));
    assert!(value_map.contains_key("humn"));

    value_map.remove("humn");

    let active = value_map.keys().cloned().collect();
    calc(&mut value_map, &dep_map, &job_map, &in_map, active);

    assert!(value_map.contains_key(in_map["root"].1));

    let mut y = value_map[in_map["root"].1];
    let mut x_node = in_map["root"].0;

    while x_node != "humn" {
        assert!(!value_map.contains_key(x_node));
        if let Some(x) = value_map.get(in_map[x_node].0) {
            y = match job_map[x_node] {
                Job::A => y - x,
                Job::S => x - y,
                Job::M => y / x,
                Job::D => x / y,
            };
            x_node = in_map[x_node].1;
            continue;
        }
        if let Some(x) = value_map.get(in_map[x_node].1) {
            y = match job_map[x_node] {
                Job::A => y - x,
                Job::S => y + x,
                Job::M => y / x,
                Job::D => y * x,
            };

            x_node = in_map[x_node].0;
            continue;
        }
    }

    println!("{y}");
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/21/example").unwrap());
    solve_part_one(&read_lines("src/21/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/21/example").unwrap());
    solve_part_two(&read_lines("src/21/input").unwrap());
}
