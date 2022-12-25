use itertools::Itertools;

fn construct_forest(input: &str) -> (usize, usize, Vec<Vec<isize>>) {
    let forest = input
        .lines()
        .map(|x| x.chars().map(|y| y as isize - '0' as isize).collect_vec())
        .collect_vec();

    (forest[0].len(), forest.len(), forest)
}

pub fn p1(input: &str) -> String {
    let (width, height, forest) = construct_forest(input);
    (0..width)
        .cartesian_product(0..height)
        .filter(|&(x, y)| {
            forest[y][x] > (0..x).map(|nx| forest[y][nx]).max().unwrap_or(-1)
                || forest[y][x] > (x + 1..width).map(|nx| forest[y][nx]).max().unwrap_or(-1)
                || forest[y][x] > (0..y).map(|ny| forest[ny][x]).max().unwrap_or(-1)
                || forest[y][x] > (y + 1..height).map(|ny| forest[ny][x]).max().unwrap_or(-1)
        })
        .count()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let (width, height, forest) = construct_forest(input);
    (0..width)
        .cartesian_product(0..height)
        .map(
            |(x, y)| {
                (0..x)
                    .rev()
                    .position(|nx| forest[y][nx] >= forest[y][x])
                    .unwrap_or_else(|| x.wrapping_sub(1))
                    .wrapping_add(1)
                    * (x + 1..width)
                        .position(|nx| forest[y][nx] >= forest[y][x])
                        .unwrap_or_else(|| (width - x - 1).wrapping_sub(1))
                        .wrapping_add(1)
                    * (0..y)
                        .rev()
                        .position(|ny| forest[ny][x] >= forest[y][x])
                        .unwrap_or_else(|| y.wrapping_sub(1))
                        .wrapping_add(1)
                    * (y + 1..height)
                        .position(|ny| forest[ny][x] >= forest[y][x])
                        .unwrap_or_else(|| (height - y - 1).wrapping_sub(1))
                        .wrapping_add(1)
            }, //}
        )
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d08_example.txt")), "21");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d08.txt")), "1845");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d08_example.txt")), "8");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d08.txt")), "230112");
}
