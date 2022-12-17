fn init_stacks(stacks_str: &str) -> Vec<Vec<char>> {
    let num_stacks = stacks_str
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stacks_str.lines().rev().skip(1) {
        let mut line = line.chars().skip(1).step_by(4);

        for stack in stacks.iter_mut() {
            if let Some(label) = line.next() {
                if label.is_alphabetic() {
                    stack.push(label);
                }
            }
        }
    }

    stacks
}

fn parse_instruction(instruction: &str) -> (u8, u8, u8) {
    let mut instruction = instruction
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|x| x.parse::<u8>().unwrap());

    (
        instruction.next().unwrap(),
        instruction.next().unwrap(),
        instruction.next().unwrap(),
    )
}

fn execute_instructions(stacks: &mut [Vec<char>], instructions: &str) {
    for instruction in instructions.lines() {
        let (moves, from, to) = parse_instruction(instruction);
        for _ in 0..moves {
            let moved_crate = stacks[from as usize - 1].pop().unwrap();
            stacks[to as usize - 1].push(moved_crate);
        }
    }
}

fn get_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut tops = String::new();

    for stack in stacks {
        tops.push(*stack.last().unwrap());
    }

    tops
}

fn main() {
    let (stacks_str, instructions) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut stacks = init_stacks(stacks_str);

    execute_instructions(&mut stacks, instructions);

    println!("{}", get_tops(&stacks));
}
