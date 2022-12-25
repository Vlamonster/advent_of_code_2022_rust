use std::collections::HashSet;

pub fn p1(input: &str) -> String {
    input
        .as_bytes()
        .windows(4)
        .position(|window| HashSet::<&u8>::from_iter(window).len() == 4)
        .unwrap()
        .wrapping_add(4)
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .as_bytes()
        .windows(14)
        .position(|window| HashSet::<&u8>::from_iter(window).len() == 14)
        .unwrap()
        .wrapping_add(14)
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d06_example.txt")), "5");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d06.txt")), "1480");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d06_example.txt")), "23");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d06.txt")), "2746");
}
