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

    for y in 0..6 {
        for x in 1..=40 {
            if (x - 2..=x).contains(&x_trace[y * 40 + x as usize]) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
