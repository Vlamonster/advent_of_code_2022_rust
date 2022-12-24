use itertools::Itertools;

fn solve(input: &str, crate_mover_9001: bool) -> String {
    let (stacks_str, instructions) = input.split_once("\n\n").unwrap();

    let num_stacks = stacks_str
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks = vec![vec![]; num_stacks];
    for layer in stacks_str.lines().rev().skip(1) {
        let layer = layer.chars().skip(1).step_by(4).enumerate();
        for (index, label) in layer {
            if label.is_alphabetic() {
                stacks[index].push(label);
            }
        }
    }

    let mut moved_crates = Vec::new();
    for instruction in instructions.lines() {
        let (moves, from, to) = instruction
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();

        if crate_mover_9001 {
            for _ in 0..moves {
                moved_crates.push(stacks[from - 1].pop().unwrap());
            }
            for _ in 0..moves {
                stacks[to - 1].push(moved_crates.pop().unwrap());
            }
        } else {
            for _ in 0..moves {
                moved_crates.push(stacks[from - 1].pop().unwrap());
                stacks[to - 1].push(moved_crates.pop().unwrap());
            }
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

pub fn p1(input: &str) -> String {
    solve(input, false)
}

pub fn p2(input: &str) -> String {
    solve(input, true)
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d05_example.txt")), "CMZ");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d05.txt")), "RNZLFZSJH");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d05_example.txt")), "MCD");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d05.txt")), "CNSFCGJSM");
}
