use itertools::Itertools;

pub fn p1(input: &str) -> String {
    input
        .lines()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            first.chars().find(|&item| second.contains(item)).unwrap()
        })
        .map(|misplaced_item| match misplaced_item {
            'a'..='z' => misplaced_item as usize - b'a' as usize + 1,
            'A'..='Z' => misplaced_item as usize - b'A' as usize + 27,
            _ => unreachable!(),
        })
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(elf_1, elf_2, elf_3)| {
            elf_1
                .chars()
                .find(|&item| elf_2.contains(item) && elf_3.contains(item))
                .unwrap()
        })
        .map(|badge| match badge {
            'a'..='z' => badge as usize - b'a' as usize + 1,
            'A'..='Z' => badge as usize - b'A' as usize + 27,
            _ => unreachable!(),
        })
        .sum::<usize>()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d03_example.txt")), "157");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d03.txt")), "8243");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d03_example.txt")), "70");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d03.txt")), "2631");
}
