use common::read_lines;

#[derive(Clone, Debug, Eq, PartialEq)]
enum List {
    Elem(u32),
    List(Vec<List>),
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (List::Elem(e), List::Elem(o)) => e.partial_cmp(o),
            (List::List(e), List::List(o)) => e.partial_cmp(o),
            (List::Elem(e), list) => List::List(vec![List::Elem(*e)]).partial_cmp(list),
            (list, List::Elem(e)) => list.partial_cmp(&List::List(vec![List::Elem(*e)])),
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_end(s: &str) -> usize {
    let mut open = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => open += 1,
            ']' => open -= 1,
            _ => {}
        };
        if open == 0 {
            return i;
        }
    }
    unreachable!()
}

struct MySplit<'a>(&'a str);

impl<'a> Iterator for MySplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let result_end = if self.0.starts_with("[") {
            find_end(&self.0) + 1
        } else {
            self.0.find(",").or(Some(self.0.len())).unwrap()
        };

        let result = &self.0[0..result_end];
        self.0 = if result_end == self.0.len() {
            ""
        } else {
            &self.0[result_end + 1..]
        };

        Some(result)
    }
}

fn parse(s: &str) -> List {
    if !s.starts_with("[") {
        List::Elem(s.parse().unwrap())
    } else {
        List::List(MySplit(&s[1..s.len() - 1]).map(parse).collect())
    }
}

fn solve_part_one(lines: &[String]) {
    println!(
        "{}",
        lines
            .split(|l| l.is_empty())
            .enumerate()
            .filter(|(_, block)| parse(&block[0]).le(&parse(&block[1])))
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );
}

fn solve_part_two(lines: &[String]) {
    let decoders = [
        List::List(vec![List::List(vec![List::Elem(2)])]),
        List::List(vec![List::List(vec![List::Elem(6)])]),
    ];

    let mut lists: Vec<List> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| parse(l))
        .chain(decoders.iter().cloned())
        .collect();

    lists.sort();
    println!(
        "{}",
        decoders
            .iter()
            .map(|d| lists.iter().position(|l| l == d).unwrap() + 1)
            .product::<usize>()
    );
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/13/example").unwrap());
    solve_part_one(&read_lines("src/13/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/13/example").unwrap());
    solve_part_two(&read_lines("src/13/input").unwrap());
}
