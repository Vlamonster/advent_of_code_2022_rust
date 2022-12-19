use itertools::Itertools;

fn main() {
    print!(
        "{:?}",
        include_str!("input.txt")
            .lines()
            .tuples::<(_, _, _)>()
            .map(|(elf_1, elf_2, elf_3)| elf_1
                .chars()
                .find(|item| elf_2.contains(*item) && elf_3.contains(*item))
                .unwrap())
            .map(|badge| match badge {
                'a'..='z' => badge as usize - b'a' as usize + 1,
                'A'..='Z' => badge as usize - b'A' as usize + 27,
                _ => unreachable!(),
            })
            .sum::<usize>()
    );
}
