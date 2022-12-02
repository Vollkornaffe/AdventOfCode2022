use common::read_lines;
use std::path::Path;

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Goal {
    Loose,
    Draw,
    Win,
}

fn map_move(s: &str) -> Move {
    match s {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => unreachable!(),
    }
}

fn map_goal(s: &str) -> Goal {
    match s {
        "X" => Goal::Loose,
        "Y" => Goal::Draw,
        "Z" => Goal::Win,
        _ => unreachable!(),
    }
}

fn map_move_goal(m: Move, g: Goal) -> Move {
    match m {
        Move::Rock => match g {
            Goal::Loose => Move::Scissors,
            Goal::Draw => Move::Rock,
            Goal::Win => Move::Paper,
        },
        Move::Paper => match g {
            Goal::Loose => Move::Rock,
            Goal::Draw => Move::Paper,
            Goal::Win => Move::Scissors,
        },
        Move::Scissors => match g {
            Goal::Loose => Move::Paper,
            Goal::Draw => Move::Scissors,
            Goal::Win => Move::Rock,
        },
    }
}

fn score_match(other: Move, me: Move) -> usize {
    const LOOSE: usize = 0;
    const DRAW: usize = 3;
    const WIN: usize = 6;

    0 + match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    } + match (other, me) {
        (Move::Rock, Move::Rock) => DRAW,
        (Move::Rock, Move::Paper) => WIN,
        (Move::Rock, Move::Scissors) => LOOSE,
        (Move::Paper, Move::Rock) => LOOSE,
        (Move::Paper, Move::Paper) => DRAW,
        (Move::Paper, Move::Scissors) => WIN,
        (Move::Scissors, Move::Rock) => WIN,
        (Move::Scissors, Move::Paper) => LOOSE,
        (Move::Scissors, Move::Scissors) => DRAW,
    }
}

fn calc_score_part_one<P: AsRef<Path>>(filename: P) -> usize {
    read_lines(filename).unwrap().iter().fold(0, |score, line| {
        let mut split = line.split(" ");
        score
            + score_match(
                map_move(split.next().unwrap()),
                map_move(split.next().unwrap()),
            )
    })
}

fn calc_score_part_two<P: AsRef<Path>>(filename: P) -> usize {
    read_lines(filename).unwrap().iter().fold(0, |score, line| {
        let mut split = line.split(" ");
        let other = map_move(split.next().unwrap());
        let goal = map_goal(split.next().unwrap());
        let me = map_move_goal(other, goal);
        score + score_match(other, me)
    })
}

fn main() {
    println!("Part one:");
    println!("{}", calc_score_part_one("src/02/example"));
    println!("{}", calc_score_part_one("src/02/input"));

    println!("Part two:");
    println!("{}", calc_score_part_two("src/02/example"));
    println!("{}", calc_score_part_two("src/02/input"));
}
