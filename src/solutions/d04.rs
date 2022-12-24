pub fn p1(input: &str) -> String {
    input
        .lines()
        .filter(|pair| {
            let (first, second) = pair.split_once(',').unwrap();
            let (a, b) = first.split_once('-').unwrap();
            let (c, d) = second.split_once('-').unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            let c = c.parse::<usize>().unwrap();
            let d = d.parse::<usize>().unwrap();
            (a >= c && b <= d) || (a <= c && b >= d)
        })
        .count()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
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
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d04_example.txt")), "2");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d04.txt")), "644");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d04_example.txt")), "4");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d04.txt")), "926");
}
