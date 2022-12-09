use common::read_lines;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug, Default)]
struct Directory {
    files: Vec<usize>,
    children: Vec<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn sum_files(&self) -> usize {
        self.files.iter().sum::<usize>()
            + self
                .children
                .iter()
                .map(|c| c.borrow().sum_files())
                .sum::<usize>()
    }
}

struct Path(Vec<Rc<RefCell<Directory>>>);

impl Path {
    fn new() -> Self {
        Self(vec![Rc::new(RefCell::new(Directory::default()))])
    }

    fn current(&self) -> RefMut<Directory> {
        self.0.last().unwrap().borrow_mut()
    }

    fn move_up(&mut self) {
        self.0.pop().unwrap();
    }

    fn add_dir(&mut self) -> Rc<RefCell<Directory>> {
        let new_dir = Rc::new(RefCell::new(Directory::default()));
        self.current().children.push(new_dir.clone());
        self.0.push(new_dir.clone());
        new_dir
    }
}

fn parse_to_dirs(lines: &[String]) -> Vec<Rc<RefCell<Directory>>> {
    let mut it = lines.iter().peekable();

    assert!(it.next().unwrap().as_str() == "$ cd /");

    let mut path = Path::new();
    let mut dirs = vec![path.0.first().unwrap().clone()];

    while it.peek().is_some() {
        match it.next().unwrap().as_str() {
            "$ cd .." => path.move_up(),
            "$ ls" => {
                while it.peek().filter(|l| !l.starts_with("$")).is_some() {
                    let line = it.next().unwrap();
                    if line.starts_with("dir") {
                        continue;
                    }
                    path.current()
                        .files
                        .push(line.split(" ").next().unwrap().parse().unwrap());
                }
            }
            _ => dirs.push(path.add_dir()),
        }
    }
    dirs
}

fn solve_part_one(lines: &[String]) {
    println!(
        "{}",
        parse_to_dirs(lines)
            .iter()
            .filter_map(|dir| {
                let dir_size = dir.borrow().sum_files();
                if dir_size > 100_000 {
                    None
                } else {
                    Some(dir_size)
                }
            })
            .sum::<usize>()
    );
}

fn solve_part_two(lines: &[String]) {
    let dirs = parse_to_dirs(lines);
    let size = 70_000_000;
    let needed = 30_000_000;
    let used = dirs.first().unwrap().borrow().sum_files();
    let free = size - used;
    let to_free = needed - free;

    let mut best_fit = size;
    for dir in dirs {
        let dir_size = dir.borrow().sum_files();
        if dir_size < to_free {
            continue;
        }
        best_fit = best_fit.min(dir_size);
    }

    println!("{}", best_fit);
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/07/example").unwrap());
    solve_part_one(&read_lines("src/07/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/07/example").unwrap());
    solve_part_two(&read_lines("src/07/input").unwrap());
}
