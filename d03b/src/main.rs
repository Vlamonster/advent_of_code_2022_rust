fn priority(item: char) -> u8 {
    if item.is_ascii_lowercase() {
        item as u8 - b'a' + 1
    } else {
        item as u8 - b'A' + 27
    }
}

fn intersection(group: &[&str]) -> Option<char> {
    group[0]
        .chars()
        .find(|&item| group[1].contains(item) && group[2].contains(item))
}

fn main() {
    print!(
        "{:?}",
        include_str!("input.txt")
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|group| priority(intersection(group).unwrap()) as usize)
            .sum::<usize>()
    );
}
