use std::fs::File;
use std::io::{self, stdin, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}

pub fn wait() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
