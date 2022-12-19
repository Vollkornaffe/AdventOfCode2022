use common::read_lines;

struct Blueprint {
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

fn step(costs: &[[usize; 3]; 4], t: usize, resources: [usize; 4], robots: [usize; 4]) -> usize {
    if t == 0 {
        //print!("{},", resources[3]);
        return resources[3];
    }
    costs
        .iter()
        .enumerate()
        .filter(|(_, &[ore, clay, obsidian])| {
            resources[0] >= ore && resources[1] >= clay && resources[2] >= obsidian
        })
        .map(|(i, &[ore, clay, obsidian])| {
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
            )
        })
        .max()
        .or(Some(step(
            costs,
            t - 1,
            [
                resources[0] + robots[0],
                resources[1] + robots[1],
                resources[2] + robots[2],
                resources[3] + robots[3],
            ],
            robots,
        )))
        .unwrap()
}

fn main() {
    println!(
        "{}",
        read_lines("src/19/example")
            .unwrap()
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
            .map(|costs| step(&costs, 22, [0; 4], [1, 0, 0, 0]))
            .enumerate()
            .map(|(i, geodes)| (i + 1) * geodes)
            .sum::<usize>()
    );
}
