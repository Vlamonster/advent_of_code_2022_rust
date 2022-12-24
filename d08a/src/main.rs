use itertools::Itertools;

fn main() {
    let forest = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().map(|y| y as isize - '0' as isize).collect_vec())
        .collect_vec();

    let width = forest[0].len();
    let height = forest.len();

    print!(
        "{}",
        (0..width)
            .cartesian_product(0..height)
            .filter(|&(x, y)| {
                forest[y][x] > (0..x).map(|nx| forest[y][nx]).max().unwrap_or(-1)
                    || forest[y][x] > (x + 1..width).map(|nx| forest[y][nx]).max().unwrap_or(-1)
                    || forest[y][x] > (0..y).map(|ny| forest[ny][x]).max().unwrap_or(-1)
                    || forest[y][x] > (y + 1..height).map(|ny| forest[ny][x]).max().unwrap_or(-1)
            })
            .count()
    );
}
