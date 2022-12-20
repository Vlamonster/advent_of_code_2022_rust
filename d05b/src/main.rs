use itertools::Itertools;

fn parse_stacks(stacks_str: &str) -> Vec<Vec<char>> {
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
    stacks
}

fn execute_instructions(stacks: &mut [Vec<char>], instructions: &str) {
    for instruction in instructions.lines() {
        let (moves, from, to) = instruction
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();

        let mut moved_crates = Vec::new();
        for _ in 0..moves {
            moved_crates.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..moves {
            stacks[to - 1].push(moved_crates.pop().unwrap());
        }
    }
}

fn main() {
    let (stacks_str, instructions) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks_str);
    execute_instructions(&mut stacks, instructions);

    print!(
        "{}",
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    )
}
