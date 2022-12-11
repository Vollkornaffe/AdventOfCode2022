use common::read_lines;

enum Op {
    AddOld,
    MulOld,
    AddConst(usize),
    MulConst(usize),
}

struct Monkey {
    items: Vec<usize>,
    items2: Vec<ItemRepresentations>,
    operation: Op,
    divisor: usize,
    monkey_true: usize,
    monkey_false: usize,
    inspections: usize,
}

impl Monkey {
    fn new(block: &[String]) -> Self {
        let items = (&block[1][18..])
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let op_char = block[2].chars().nth(23).unwrap();
        let operation = match &block[2][25..] {
            "old" => match op_char {
                '+' => Op::AddOld,
                '*' => Op::MulOld,
                _ => unreachable!(),
            },
            i => {
                let i = i.parse().unwrap();
                match op_char {
                    '+' => Op::AddConst(i),
                    '*' => Op::MulConst(i),
                    _ => unreachable!(),
                }
            }
        };

        let divisor = block[3][21..].parse().unwrap();
        let monkey_true = block[4][29..].parse().unwrap();
        let monkey_false = block[5][30..].parse().unwrap();

        Self {
            items,
            items2: Vec::new(), // can only be filled after all monkeys are parsed
            operation,
            divisor,
            monkey_true,
            monkey_false,
            inspections: 0,
        }
    }
}

struct ItemRepresentations(Vec<(usize, usize)>);

impl ItemRepresentations {
    fn new(divisors: &[usize], item: usize) -> Self {
        Self(divisors.iter().map(|&d| (item % d, d)).collect())
    }

    fn update(self, op: &Op) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(item, d)| {
                    (
                        match op {
                            Op::AddOld => item + item,
                            Op::MulOld => item * item,
                            Op::AddConst(k) => item + k,
                            Op::MulConst(k) => item * k,
                        } % d,
                        d,
                    )
                })
                .collect(),
        )
    }
}

fn parse_monkeys(lines: &[String]) -> Vec<Monkey> {
    lines.split(|l| l.is_empty()).map(Monkey::new).collect()
}

fn calc_monkey_business(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    monkeys[0].inspections * monkeys[1].inspections
}

fn solve_part_one(lines: &[String]) {
    let mut monkeys = parse_monkeys(lines);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for old in monkeys[i].items.drain(..).collect::<Vec<_>>().drain(..) {
                monkeys[i].inspections += 1;
                let new = match monkeys[i].operation {
                    Op::AddOld => old + old,
                    Op::MulOld => old * old,
                    Op::AddConst(k) => old + k,
                    Op::MulConst(k) => old * k,
                } / 3;
                let target = if new % monkeys[i].divisor == 0 {
                    monkeys[i].monkey_true
                } else {
                    monkeys[i].monkey_false
                };
                monkeys[target].items.push(new);
            }
        }
    }

    println!("{}", calc_monkey_business(monkeys));
}

fn solve_part_two(lines: &[String]) {
    let mut monkeys = parse_monkeys(lines);
    {
        let divisors: Vec<_> = monkeys.iter().map(|m| m.divisor).collect();
        monkeys.iter_mut().for_each(|m| {
            m.items2 = m
                .items
                .drain(..)
                .collect::<Vec<_>>()
                .drain(..)
                .map(|item| ItemRepresentations::new(&divisors, item))
                .collect();
        });
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for old in monkeys[i].items2.drain(..).collect::<Vec<_>>().drain(..) {
                monkeys[i].inspections += 1;
                let new = old.update(&monkeys[i].operation);
                let target = if new.0[i].0 == 0 {
                    monkeys[i].monkey_true
                } else {
                    monkeys[i].monkey_false
                };
                monkeys[target].items2.push(new);
            }
        }
    }

    println!("{}", calc_monkey_business(monkeys));
}

fn main() {
    println!("Part one:");
    solve_part_one(&read_lines("src/11/example").unwrap());
    solve_part_one(&read_lines("src/11/input").unwrap());

    println!("Part two:");
    solve_part_two(&read_lines("src/11/example").unwrap());
    solve_part_two(&read_lines("src/11/input").unwrap());
}
