fn main() {
    print!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|elf| elf
                .lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum::<usize>())
            .max()
            .unwrap()
    );
}
