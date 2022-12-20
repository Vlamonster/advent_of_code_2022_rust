fn main() {
    let mut x_trace = vec![0; 1];
    let mut x = 1;

    for instruction in include_str!("input.txt").lines() {
        if instruction == "noop" {
            x_trace.push(x);
        } else {
            let (_, operand) = instruction.split_once(' ').unwrap();
            x_trace.push(x);
            x_trace.push(x);
            x += operand.parse::<isize>().unwrap();
        }
    }

    print!(
        "{}",
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|cycle| x_trace[*cycle as usize] * cycle)
            .sum::<isize>()
    );
}
