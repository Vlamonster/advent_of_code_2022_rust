use itertools::Itertools;

fn mix(file: &mut Vec<(usize, isize)>) {
    for index in 0..file.len() {
        let pos = file.iter().position(|&(x, _)| x == index).unwrap();
        let (i, value) = file.remove(pos);
        let new_index = (pos as isize + value).rem_euclid(file.len() as isize) as usize;
        file.insert(new_index, (i, value));
    }
}

pub fn p1(input: &str) -> String {
    let mut file = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .enumerate()
        .collect_vec();
    mix(&mut file);
    let zero = file.iter().position(|x| x.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|x| file[(zero + x) % file.len()].1)
        .sum::<isize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let mut file = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap() * 811589153)
        .enumerate()
        .collect_vec();
    for _ in 0..10 {
        mix(&mut file);
    }
    let zero = file.iter().position(|x| x.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|x| file[(zero + x) % file.len()].1)
        .sum::<isize>()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d20_example.txt")), "3");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d20.txt")), "4151");
}

#[test]
fn p2_example() {
    assert_eq!(
        p2(include_str!("../../inputs/d20_example.txt")),
        "1623178306"
    );
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d20.txt")), "7848878698663");
}
