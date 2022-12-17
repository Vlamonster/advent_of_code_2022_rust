fn priority(item: char) -> u8 {
    if item.is_ascii_lowercase() {
        item as u8 - b'a' + 1
    } else {
        item as u8 - b'A' + 27
    }
}

fn intersection(rucksack: &str) -> Option<char> {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);
    first.chars().find(|&item| second.contains(item))
}

fn main() {
    print!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|rucksack| priority(intersection(rucksack).unwrap()) as usize)
            .sum::<usize>()
    );
}
