use crate::Operation::{Add, Mul, Square};
use itertools::Itertools;

enum Operation {
    Square,
    Mul(usize),
    Add(usize),
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for monkey_data in include_str!("input.txt").split("\n\n") {
        let mut monkey_data = monkey_data.lines().skip(1);

        let items = monkey_data
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect_vec();

        let operation = match monkey_data
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .unwrap()
        {
            ("*", "old") => Square,
            ("*", operand) => Mul(operand.parse().unwrap()),
            ("+", operand) => Add(operand.parse().unwrap()),
            _ => unreachable!(),
        };

        let test = monkey_data
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let test_true = monkey_data
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let test_false = monkey_data
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            test_true,
            test_false,
            inspections: 0,
        })
    }
    monkeys
}

fn main() {
    let mut monkeys = parse_monkeys();
    let lcm = monkeys.iter().map(|monkey| monkey.test).product::<usize>(); // technically not lcm, but it works

    for _ in 0..10000 {
        for monkey_id in 0..monkeys.len() {
            while let Some(mut item) = monkeys[monkey_id].items.pop() {
                monkeys[monkey_id].inspections += 1;

                item = match monkeys[monkey_id].operation {
                    Square => item * item,
                    Mul(operand) => item * operand,
                    Add(operand) => item + operand,
                } % lcm;

                let target_monkey = if item % monkeys[monkey_id].test == 0 {
                    monkeys[monkey_id].test_true
                } else {
                    monkeys[monkey_id].test_false
                };

                monkeys[target_monkey].items.push(item);
            }
        }
    }

    print!(
        "{:?}",
        monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    );
}
