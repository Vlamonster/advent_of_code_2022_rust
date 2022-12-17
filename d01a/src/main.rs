fn main() {
    print!(
        "{}",
        include_str!("input.txt")
            .split("\n\n")
            .map(|cal_per_elf| cal_per_elf
                .lines()
                .map(|cal_per_item| cal_per_item.parse::<usize>().unwrap())
                .sum::<usize>())
            .max()
            .unwrap()
    );
}
