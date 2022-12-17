fn main() {
    let mut cal_per_elf = include_str!("input.txt")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|y| y.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    cal_per_elf.sort();
    println!("{}", cal_per_elf.into_iter().rev().take(3).sum::<usize>());
}
