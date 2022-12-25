use itertools::Itertools;
use num::Integer;
use Operation::{Add, Mul, Square};

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

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for monkey_data in input.split("\n\n") {
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

fn play_keep_away(
    monkeys: &mut Vec<Monkey>,
    steps: usize,
    relieve: Box<dyn Fn(usize) -> usize>,
) -> usize {
    for _ in 0..steps {
        for monkey_id in 0..monkeys.len() {
            while let Some(mut item) = monkeys[monkey_id].items.pop() {
                monkeys[monkey_id].inspections += 1;

                item = relieve(match monkeys[monkey_id].operation {
                    Square => item * item,
                    Mul(operand) => item * operand,
                    Add(operand) => item + operand,
                });

                let target_monkey = if item % monkeys[monkey_id].test == 0 {
                    monkeys[monkey_id].test_true
                } else {
                    monkeys[monkey_id].test_false
                };

                monkeys[target_monkey].items.push(item);
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

pub fn p1(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);
    play_keep_away(&mut monkeys, 20, Box::new(|x| x / 3)).to_string()
}

pub fn p2(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);
    let lcm = monkeys.iter().fold(1, |acc, monkey| acc.lcm(&monkey.test));
    play_keep_away(&mut monkeys, 10000, Box::new(move |x| x % lcm)).to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d11_example.txt")), "10605");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d11.txt")), "54036");
}

#[test]
fn p2_example() {
    assert_eq!(
        p2(include_str!("../../inputs/d11_example.txt")),
        "2713310158"
    );
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d11.txt")), "13237873355");
}
