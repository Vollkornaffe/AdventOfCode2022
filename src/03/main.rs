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

fn main() {
    println!("{}", solve_part_one("src/03/example"));
    println!("{}", solve_part_one("src/03/input"));
}
