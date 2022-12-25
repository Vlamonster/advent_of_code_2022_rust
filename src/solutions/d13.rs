use itertools::Itertools;
use std::cmp::Ordering;
use std::str::Chars;
use Node::{Array, Number};

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

pub fn p1(input: &str) -> String {
    input
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| {
            let (first, second) = pair.split_once('\n').unwrap();
            parse(first) < parse(second)
        })
        .map(|(index, _)| index + 1)
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .chain(["[[2]]", "[[6]]"])
        .map(parse)
        .sorted()
        .enumerate()
        .filter(|(_, y)| *y == parse("[[2]]") || *y == parse("[[6]]"))
        .map(|(x, _)| x + 1)
        .product::<usize>()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d13_example.txt")), "13");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d13.txt")), "6272");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d13_example.txt")), "140");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d13.txt")), "22288");
}
