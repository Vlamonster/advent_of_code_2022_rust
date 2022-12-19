use crate::Operation::{Add, Double, Mul, Square};
use regex::Regex;

enum Operation {
    Square,
    Double,
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

impl Monkey {
    pub fn from_information(information: &str) -> Self {
        let integer_re = Regex::new(r"\d+").unwrap();
        let equation_re = Regex::new(r"old|\d+|[*+]").unwrap();

        let mut lines = information.lines().skip(1);
        let items = integer_re
            .find_iter(lines.next().unwrap())
            .map(|re_match| re_match.as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut operation_matches = equation_re.find_iter(lines.next().unwrap()).skip(1);
        let operation = match operation_matches.next().unwrap().as_str() {
            "+" => match operation_matches.next().unwrap().as_str() {
                "old" => Double,
                operand => Add(operand.parse::<usize>().unwrap()),
            },
            "*" => match operation_matches.next().unwrap().as_str() {
                "old" => Square,
                operand => Mul(operand.parse::<usize>().unwrap()),
            },
            _ => unreachable!(),
        };
        let test = integer_re
            .find(lines.next().unwrap())
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let test_true = integer_re
            .find(lines.next().unwrap())
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let test_false = integer_re
            .find(lines.next().unwrap())
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        Monkey {
            items,
            operation,
            test,
            test_true,
            test_false,
            inspections: 0,
        }
    }

    pub fn inspect_items(&mut self, lcm: usize) {
        for item in &mut self.items {
            self.inspections += 1;
            *item = match self.operation {
                Square => *item * *item,
                Double => *item + *item,
                Mul(operand) => *item * operand,
                Add(operand) => *item + operand,
            } % lcm;
        }
    }

    pub fn test_worry(&self, item: &usize) -> usize {
        if item % self.test == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }
}

fn do_round(monkeys: &mut Vec<Monkey>, lcm: usize) {
    for monkey_id in 0..monkeys.len() {
        monkeys[monkey_id].inspect_items(lcm);
        let items = monkeys[monkey_id].items.clone();
        for item in &items {
            let new_monkey = monkeys[monkey_id].test_worry(item);
            monkeys[new_monkey].items.push(*item);
        }
        monkeys[monkey_id].items.clear();
    }
}

fn main() {
    let mut monkeys = Vec::new();
    for information in include_str!("input.txt").split("\n\n") {
        monkeys.push(Monkey::from_information(information));
    }

    // technically not lcm, but it works
    let lcm = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..10000 {
        do_round(&mut monkeys, lcm);
    }

    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    print!("{}", inspections.pop().unwrap() * inspections.pop().unwrap());
}
