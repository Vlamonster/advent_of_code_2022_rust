use crate::solutions::d09::Motion::{Down, Left, Right, Up};
use std::collections::HashSet;

enum Motion {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

struct Rope {
    knots: Vec<(isize, isize)>,
    length: usize,
    visited: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        Rope {
            knots: vec![(0, 0); knots],
            length: knots,
            visited: HashSet::from_iter([(0, 0)]),
        }
    }

    pub fn move_head(&mut self, motion: Motion) {
        match motion {
            Up(steps) => {
                for _ in 0..steps {
                    self.knots[0].1 += 1;
                    for tail_knot in 1..self.length {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(*self.knots.last().unwrap());
                }
            }
            Down(steps) => {
                for _ in 0..steps {
                    self.knots[0].1 -= 1;
                    for tail_knot in 1..self.length {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(*self.knots.last().unwrap());
                }
            }
            Left(steps) => {
                for _ in 0..steps {
                    self.knots[0].0 -= 1;
                    for tail_knot in 1..self.length {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(*self.knots.last().unwrap());
                }
            }
            Right(steps) => {
                for _ in 0..steps {
                    self.knots[0].0 += 1;
                    for tail_knot in 1..self.length {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(*self.knots.last().unwrap());
                }
            }
        }
    }

    fn move_tail(&mut self, knot: usize) {
        let x_diff = self.knots[knot - 1].0 - self.knots[knot].0;
        let y_diff = self.knots[knot - 1].1 - self.knots[knot].1;

        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
            self.knots[knot].0 += x_diff.signum();
            self.knots[knot].1 += y_diff.signum();
        }
    }
}

fn find_tail_locations(input: &str, knots: usize) -> usize {
    let mut rope = Rope::new(knots);

    for instruction in input.lines() {
        let (motion, steps) = instruction.split_once(' ').unwrap();
        let motion = match motion {
            "U" => Up(steps.parse::<isize>().unwrap()),
            "D" => Down(steps.parse::<isize>().unwrap()),
            "L" => Left(steps.parse::<isize>().unwrap()),
            "R" => Right(steps.parse::<isize>().unwrap()),
            _ => unreachable!(),
        };
        rope.move_head(motion);
    }

    rope.visited.len()
}

pub fn p1(input: &str) -> String {
    find_tail_locations(input, 2).to_string()
}

pub fn p2(input: &str) -> String {
    find_tail_locations(input, 10).to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d09_example_1.txt")), "13");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d09.txt")), "6563");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d09_example_2.txt")), "36");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d09.txt")), "2653");
}
