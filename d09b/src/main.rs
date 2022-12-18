use crate::Motion::{Down, Left, Right, Up};
use std::collections::HashSet;

enum Motion {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            knots: vec![(0, 0); 10],
            visited: HashSet::from_iter([(0, 0)]),
        }
    }

    pub fn move_head(&mut self, motion: Motion) {
        match motion {
            Up(steps) => {
                for _ in 0..steps {
                    self.knots[0].1 += 1;
                    for tail_knot in 1..10 {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(self.knots[9]);
                }
            }
            Down(steps) => {
                for _ in 0..steps {
                    self.knots[0].1 -= 1;
                    for tail_knot in 1..10 {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(self.knots[9]);
                }
            }
            Left(steps) => {
                for _ in 0..steps {
                    self.knots[0].0 -= 1;
                    for tail_knot in 1..10 {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(self.knots[9]);
                }
            }
            Right(steps) => {
                for _ in 0..steps {
                    self.knots[0].0 += 1;
                    for tail_knot in 1..10 {
                        self.move_tail(tail_knot);
                    }
                    self.visited.insert(self.knots[9]);
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

fn main() {
    let mut rope = Rope::new();

    for instruction in include_str!("input.txt").lines() {
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

    print!("{:?}", rope.visited.len());
}
