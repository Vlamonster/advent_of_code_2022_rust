use crate::Node::{Array, Number};
use std::cmp::Ordering;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Array(Vec<Node>),
    Number(usize),
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number(left), Number(right)) => left.cmp(right),
            (Array(left), Array(right)) => left.iter().cmp(right),
            (Number(left), Array(_)) => Array(vec![Number(*left)]).cmp(other),
            (Array(_), Number(right)) => self.cmp(&Array(vec![Number(*right)])),
        }
    }
}

fn parse(input: &str) -> Node {
    let mut to_parse = input.chars();
    to_parse.next();
    parse_recursively(&mut to_parse)
}

fn parse_recursively(to_parse: &mut Chars) -> Node {
    let mut result = Vec::new();
    let mut number = None;
    while let Some(char) = to_parse.next() {
        match char {
            '[' => result.push(parse_recursively(to_parse)),
            ',' => {
                if let Some(value) = number.take() {
                    result.push(Number(value as usize));
                }
            }
            '0'..='9' => match number {
                None => number = Some((char as u8 - b'0') as isize),
                Some(value) => number = Some(value * 10 + (char as u8 - b'0') as isize),
            },
            ']' => {
                if let Some(value) = number {
                    result.push(Number(value as usize))
                }
                return Array(result);
            }
            _ => unreachable!(),
        }
    }
    Array(result)
}

fn main() {
    print!(
        "{:?}",
        include_str!("input.txt")
            .split("\n\n")
            .enumerate()
            .filter(|(_, pair)| {
                let (first, second) = pair.split_once('\n').unwrap();
                parse(first) < parse(second)
            })
            .map(|(index, _)| index + 1)
            .sum::<usize>()
    );
}
