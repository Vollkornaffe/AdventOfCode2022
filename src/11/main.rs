use common::read_lines;

enum Op {
    AddOld,
    MulOld,
    AddConst(usize),
    MulConst(usize),
}

struct Monkey {
    items: Vec<usize>,
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
            operation,
            divisor,
            monkey_true,
            monkey_false,
            inspections: 0,
        }
    }
}

fn solve_part_one(lines: &[String]) {
    let mut monkeys: Vec<Monkey> = lines.split(|l| l.is_empty()).map(Monkey::new).collect();

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

    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();

    println!("{}", monkeys[0].inspections * monkeys[1].inspections);
}

fn main() {
    solve_part_one(&read_lines("src/11/example").unwrap());
    solve_part_one(&read_lines("src/11/input").unwrap());
}
