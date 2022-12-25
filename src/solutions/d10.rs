fn create_trace(input: &str) -> Vec<isize> {
    let mut x_trace = vec![0; 1];
    let mut x = 1;

    for instruction in input.lines() {
        if instruction == "noop" {
            x_trace.push(x);
        } else {
            let (_, operand) = instruction.split_once(' ').unwrap();
            x_trace.push(x);
            x_trace.push(x);
            x += operand.parse::<isize>().unwrap();
        }
    }

    x_trace
}

pub fn p1(input: &str) -> String {
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|cycle| create_trace(input)[*cycle as usize] * cycle)
        .sum::<isize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let trace = create_trace(input);
    let mut crt = String::new();

    for y in 0..6 {
        crt.push('\n');
        for x in 1..=40 {
            if (x - 2..=x).contains(&trace[y * 40 + x as usize]) {
                crt.push('█');
            } else {
                crt.push('•');
            }
        }
    }

    crt
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d10_example.txt")), "13140");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d10.txt")), "17180");
}

#[test]
fn p2_example() {
    assert_eq!(
        p2(include_str!("../../inputs/d10_example.txt")),
        "
██••██••██••██••██••██••██••██••██••██••
███•••███•••███•••███•••███•••███•••███•
████••••████••••████••••████••••████••••
█████•••••█████•••••█████•••••█████•••••
██████••••••██████••••••██████••••••████
███████•••••••███████•••••••███████•••••"
    );
}

#[test]
fn p2_real() {
    assert_eq!(
        p2(include_str!("../../inputs/d10.txt")),
        "
███••████•█••█•███••███••█••••█••█•███••
█••█•█••••█••█•█••█•█••█•█••••█••█•█••█•
█••█•███••████•█••█•█••█•█••••█••█•███••
███••█••••█••█•███••███••█••••█••█•█••█•
█•█••█••••█••█•█••••█•█••█••••█••█•█••█•
█••█•████•█••█•█••••█••█•████••██••███••"
    );
}
