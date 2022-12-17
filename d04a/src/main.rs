fn contains(pair: &str) -> bool {
    let (first, second) = pair.split_once(',').unwrap();
    let (a, b) = first.split_once('-').unwrap();
    let (c, d) = second.split_once('-').unwrap();

    let a = a.parse::<u8>().unwrap();
    let b = b.parse::<u8>().unwrap();
    let c = c.parse::<u8>().unwrap();
    let d = d.parse::<u8>().unwrap();

    (a >= c && b <= d) || (a <= c && b >= d)
}

fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .filter(|pair| contains(pair))
            .count()
    );
}
