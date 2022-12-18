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
    head: (isize, isize),
    tail: (isize, isize),
    visited: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            visited: HashSet::from_iter([(0, 0)]),
        }
    }

    pub fn move_head(&mut self, motion: Motion) {
        match motion {
            Up(steps) => {
                for _ in 0..steps {
                    self.head.1 += 1;
                    self.move_tail();
                }
            }
            Down(steps) => {
                for _ in 0..steps {
                    self.head.1 -= 1;
                    self.move_tail();
                }
            }
            Left(steps) => {
                for _ in 0..steps {
                    self.head.0 -= 1;
                    self.move_tail();
                }
            }
            Right(steps) => {
                for _ in 0..steps {
                    self.head.0 += 1;
                    self.move_tail();
                }
            }
        }
    }

    fn move_tail(&mut self) {
        let x_diff = self.head.0 - self.tail.0;
        let y_diff = self.head.1 - self.tail.1;

        if y_diff.abs() >= 2 || x_diff.abs() >= 2 {
            self.tail.0 += x_diff.signum();
            self.tail.1 += y_diff.signum();
        }

        self.visited.insert(self.tail);
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

    println!("{:?}", rope.visited.len());
}
