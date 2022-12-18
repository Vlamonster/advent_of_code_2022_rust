fn main() {
    let mut register_x_trace: Vec<isize> = vec![0; 2];
    let mut register_x = 1;

    for instruction in include_str!("input.txt").lines() {
        if instruction == "noop" {
            register_x_trace.push(register_x);
        } else {
            let (_, operand) = instruction.split_once(' ').unwrap();
            register_x_trace.push(register_x);
            register_x += operand.parse::<isize>().unwrap();
            register_x_trace.push(register_x);
        }
    }

    // Might be difficult to read. My solution reads "REHPRLUB".
    for y in 0..6 {
        for x in 0..40 {
            let cycle: isize = y * 40 + x;
            if (x - 2..=x).contains(&register_x_trace[cycle as usize]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
