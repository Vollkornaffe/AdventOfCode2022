use std::collections::{BTreeSet, HashMap};

use common::read_lines;

#[derive(Debug)]
struct Network {
    start: usize,
    rates: Vec<usize>,
    connections: Vec<(usize, usize)>,
    shortest: Vec<Vec<usize>>,
}

impl Network {
    fn new(lines: &[String]) -> Self {
        let mut rates = Vec::new();
        let mut connections = Vec::new();

        let mut id_map: HashMap<&str, usize> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let mut split = line.split(" ");
                (split.nth(1).unwrap(), i)
            })
            .collect();

        for (i, line) in lines.iter().enumerate() {
            let mut split = line.split(" ");

            split.next(); // Valve
            split.next(); // <ID>
            split.next(); // has
            split.next(); // flow

            let rate = split.next().unwrap().split("=").last().unwrap();
            rates.push(rate[0..rate.len() - 1].parse().unwrap());

            split.next(); // tunnels
            split.next(); // lead
            split.next(); // to
            split.next(); // valves

            connections.extend(split.map(|id| {
                (
                    i,
                    *id_map
                        .get(
                            &id[0..if id.ends_with(",") {
                                id.len() - 1
                            } else {
                                id.len()
                            }],
                        )
                        .unwrap(),
                )
            }));
        }

        let n = rates.len();

        // https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm#Algorithm
        let mut shortest = vec![vec![std::usize::MAX; n]; n];
        for i in 0..n {
            shortest[i][i] = 0;
        }
        for &(a, b) in &connections {
            shortest[a][b] = 1;
        }
        for k in 0..n {
            for j in 0..n {
                for i in 0..n {
                    if shortest[i][k] == std::usize::MAX || shortest[k][j] == std::usize::MAX {
                        continue;
                    }
                    if shortest[i][j] > shortest[i][k] + shortest[k][j] {
                        shortest[i][j] = shortest[i][k] + shortest[k][j];
                    }
                }
            }
        }

        Self {
            start: *id_map.get("AA").unwrap(),
            rates,
            connections,
            shortest,
        }
    }
}

#[derive(Clone, Debug)]
struct Bruteforce {
    time: usize,
    pressure: usize,
    opened: Vec<usize>,
    current: usize,
}

impl Bruteforce {
    pub fn step(&self, network: &Network) -> Self {
        network.shortest[self.current]
            .iter()
            .enumerate()
            .filter(|(idx, _)| network.rates[*idx] > 0 && !self.opened.contains(idx))
            .filter(|&(_, &dist)| dist < self.time)
            .map(|(idx, dist)| {
                let mut new_opened = self.opened.clone();
                new_opened.push(idx);

                let new_time = self.time - *dist - 1;

                let state = Self {
                    time: new_time,
                    pressure: self.pressure + network.rates[idx] * new_time,
                    opened: new_opened,
                    current: idx,
                };

                state.step(network)
            })
            .max_by_key(|state| state.pressure)
            .unwrap_or_else(|| self.clone())
    }
}

fn solve_part_one(mut network: Network) {
    println!("{:#?}", network);

    /*let mut current = network.start;
    let mut t = 30;

    let mut released_pressure = 0;

    while let Some((i, value)) = network.shortest[current]
        .iter()
        .enumerate()
        .filter(|&(_, &cost)| cost < t)
        .map(|(i, cost)| (i, network.rates[i] * (t - cost - 1)))
        .inspect(|x| println!("option: {:?}", x))
        .max_by_key(|(_, value)| *value)
    {
        println!("Opening {}, which is going to release {}", i, value);
        released_pressure += value;
        network.rates[i] = 0;
        println!("{} - {}", t, network.shortest[current][i]);
        t -= network.shortest[current][i] + 1;
        current = i;
    }

    println!("{}", released_pressure);*/

    println!(
        "{:#?}",
        Bruteforce {
            time: 30,
            pressure: 0,
            opened: Default::default(),
            current: network.start
        }
        .step(&network)
    );
}

fn main() {
    solve_part_one(Network::new(&read_lines("src/16/input").unwrap()));
}
