fn main() {
    print!(
        "{}",
        include_str!("input.txt")
            .lines()
            .filter(|pair| {
                let (first, second) = pair.split_once(',').unwrap();
                let (a, b) = first.split_once('-').unwrap();
                let (c, d) = second.split_once('-').unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                let c = c.parse::<usize>().unwrap();
                let d = d.parse::<usize>().unwrap();
                a <= d && c <= b
            })
            .count()
    );
}
