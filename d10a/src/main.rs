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

    print!("{}",
           register_x_trace[20] * 20
               + register_x_trace[60] * 60
               + register_x_trace[100] * 100
               + register_x_trace[140] * 140
               + register_x_trace[180] * 180
               + register_x_trace[220] * 220
    );
}
