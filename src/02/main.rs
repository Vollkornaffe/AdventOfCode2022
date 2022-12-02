use common::read_lines;
use std::path::Path;

const LOOSE: usize = 0;
const DRAW: usize = 3;
const WIN: usize = 6;

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
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

fn score_match(other: Move, me: Move) -> usize {
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

fn calc_score<P: AsRef<Path>>(filename: P) -> usize {
    read_lines(filename).unwrap().iter().fold(0, |score, line| {
        let mut split = line.split(" ");
        score
            + score_match(
                map_move(split.next().unwrap()),
                map_move(split.next().unwrap()),
            )
    })
}
fn main() {
    println!("{}", calc_score("src/02/example"));
    println!("{}", calc_score("src/02/input"));
}
