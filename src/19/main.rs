use common::read_lines;

fn upper_bound(
    costs: &[[usize; 3]; 4],
    t: usize,
    resources: [usize; 4],
    robots: [usize; 4],
) -> usize {
    resources[3] + robots[3] * t + t * (t + 1) / 2
}

fn step(
    costs: &[[usize; 3]; 4],
    t: usize,
    resources: [usize; 4],
    robots: [usize; 4],
    lower_bound: usize,
) -> usize {
    if t == 0 {
        //print!("{},", resources[3]);
        return resources[3];
    }
    if upper_bound(costs, t, resources, robots) <= lower_bound {
        return lower_bound;
    }

    let lower_bound = costs
        .iter()
        .enumerate()
        .filter(|(_, &[ore, clay, obsidian])| {
            resources[0] >= ore && resources[1] >= clay && resources[2] >= obsidian
        })
        .filter(|&(i, _)| i == 3 || costs.iter().any(|cost| cost[i] > robots[i]))
        .fold(lower_bound, |lower_bound, (i, &[ore, clay, obsidian])| {
            step(
                costs,
                t - 1,
                [
                    resources[0] + robots[0] - ore,
                    resources[1] + robots[1] - clay,
                    resources[2] + robots[2] - obsidian,
                    resources[3] + robots[3],
                ],
                {
                    let mut robots = robots;
                    robots[i] += 1;
                    robots
                },
                lower_bound,
            )
        });
    step(
        costs,
        t - 1,
        [
            resources[0] + robots[0],
            resources[1] + robots[1],
            resources[2] + robots[2],
            resources[3] + robots[3],
        ],
        robots,
        lower_bound,
    )
}

fn parse_blueprints(lines: &[String]) -> Vec<[[usize; 3]; 4]> {
    lines
        .iter()
        .map(|line| {
            let mut split = line.split(" ");

            split.next().unwrap(); //Blueprint
            split.next().unwrap(); //1:
            split.next().unwrap(); //Each
            split.next().unwrap(); //ore
            split.next().unwrap(); //robot
            split.next().unwrap(); //costs
            let ore_robot_ore_cost = split.next().unwrap().parse::<usize>().unwrap();
            split.next().unwrap(); //ore.
            split.next().unwrap(); //Each
            split.next().unwrap(); //clay
            split.next().unwrap(); //robot
            split.next().unwrap(); //costs
            let clay_robot_ore_cost = split.next().unwrap().parse::<usize>().unwrap();
            split.next().unwrap(); //ore.
            split.next().unwrap(); //Each
            split.next().unwrap(); //obsidian
            split.next().unwrap(); //robot
            split.next().unwrap(); //costs
            let obsidian_robot_ore_cost = split.next().unwrap().parse::<usize>().unwrap();
            split.next().unwrap(); //ore
            split.next().unwrap(); //and
            let obsidian_robot_clay_cost = split.next().unwrap().parse::<usize>().unwrap();
            split.next().unwrap(); //clay.
            split.next().unwrap(); //Each
            split.next().unwrap(); //geode
            split.next().unwrap(); //robot
            split.next().unwrap(); //costs
            let geode_robot_ore_cost = split.next().unwrap().parse::<usize>().unwrap();
            split.next().unwrap(); //ore
            split.next().unwrap(); //and
            let geode_robot_obsidian_cost = split.next().unwrap().parse::<usize>().unwrap();
            split.next().unwrap(); //obsidian.

            [
                [ore_robot_ore_cost, 0, 0],
                [clay_robot_ore_cost, 0, 0],
                [obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0],
                [geode_robot_ore_cost, 0, geode_robot_obsidian_cost],
            ]
        })
        .collect()
}

fn solve_part_one(lines: &[String]) -> usize {
    parse_blueprints(lines)
        .into_iter()
        .map(|costs| step(&costs, 24, [0; 4], [1, 0, 0, 0], 0))
        .enumerate()
        .inspect(|(i, geodes)| println!("{}: {}", i, geodes))
        .map(|(i, geodes)| (i + 1) * geodes)
        .sum::<usize>()
}

fn solve_part_two(lines: &[String]) -> usize {
    parse_blueprints(lines)
        .into_iter()
        .take(3)
        .map(|costs| step(&costs, 32, [0; 4], [1, 0, 0, 0], 0))
        .inspect(|geodes| println!("{geodes}"))
        .product::<usize>()
}

fn main() {
    println!("Part one:");
    println!("{}", solve_part_one(&read_lines("src/19/example").unwrap()));
    println!("{}", solve_part_one(&read_lines("src/19/input").unwrap()));

    println!("Part two:");
    println!("{}", solve_part_two(&read_lines("src/19/example").unwrap()));
    println!("{}", solve_part_two(&read_lines("src/19/input").unwrap()));
}
