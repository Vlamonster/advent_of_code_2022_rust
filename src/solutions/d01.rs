use itertools::Itertools;

pub fn p1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d01_example.txt")), "24000");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d01.txt")), "68775");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d01_example.txt")), "45000");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d01.txt")), "202585");
}
