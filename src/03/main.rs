use common::read_lines;
use std::{collections::HashSet, path::Path};

fn alphabet() -> Vec<char> {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Item(char);
impl Item {
    fn prio(&self) -> usize {
        1 + alphabet().iter().position(|&c| c == self.0).unwrap()
    }
}

struct RuckSack(HashSet<Item>, HashSet<Item>);

impl RuckSack {
    fn new(s: &String) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let mut halves = chars.as_slice().chunks(chars.len() / 2);
        Self(
            halves.next().unwrap().iter().map(|&c| Item(c)).collect(),
            halves.next().unwrap().iter().map(|&c| Item(c)).collect(),
        )
    }

    fn duplicate(&self) -> Item {
        *self.0.intersection(&self.1).next().unwrap()
    }
}

fn read_rucksacks<P: AsRef<Path>>(filename: P) -> Vec<RuckSack> {
    read_lines(filename)
        .unwrap()
        .iter()
        .map(RuckSack::new)
        .collect()
}

fn solve_part_one<P: AsRef<Path>>(filename: P) -> usize {
    read_rucksacks(filename)
        .iter()
        .map(|rs| rs.duplicate().prio())
        .sum()
}

fn solve_part_two<P: AsRef<Path>>(filename: P) -> usize {
    read_rucksacks(filename)
        .as_slice()
        .chunks(3)
        .map(|group| {
            let a: HashSet<Item> = group[0].0.union(&group[0].1).cloned().collect();
            let b: HashSet<Item> = group[1].0.union(&group[1].1).cloned().collect();
            let c: HashSet<Item> = group[2].0.union(&group[2].1).cloned().collect();
            a.intersection(&b)
                .cloned()
                .collect::<HashSet<Item>>()
                .intersection(&c)
                .cloned()
                .next()
                .unwrap()
                .prio()
        })
        .sum()
}

fn main() {
    println!("Part one:");
    println!("{}", solve_part_one("src/03/example"));
    println!("{}", solve_part_one("src/03/input"));

    println!("Part two:");
    println!("{}", solve_part_two("src/03/example"));
    println!("{}", solve_part_two("src/03/input"));
}
