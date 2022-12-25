pub fn p1(input: &str) -> String {
    input
        .lines()
        .map(|game| match game {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => unreachable!(),
        })
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .lines()
        .map(|game| match game{
            "A X" => 3,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => unreachable!(),
        })
        .sum::<usize>()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d02_example.txt")), "15");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d02.txt")), "11475");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d02_example.txt")), "12");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d02.txt")), "16862");
}
