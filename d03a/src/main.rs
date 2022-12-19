fn main() {
    print!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|rucksack| {
                let (first, second) = rucksack.split_at(rucksack.len() / 2);
                first.chars().find(|item| second.contains(*item)).unwrap()
            })
            .map(|misplaced_item| match misplaced_item {
                'a'..='z' => misplaced_item as usize - b'a' as usize + 1,
                'A'..='Z' => misplaced_item as usize - b'A' as usize + 27,
                _ => unreachable!(),
            })
            .sum::<usize>()
    );
}
