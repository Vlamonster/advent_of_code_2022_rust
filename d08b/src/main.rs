use itertools::Itertools;

fn main() {
    let forest = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().map(|y| y as isize - b'0' as isize).collect_vec())
        .collect_vec();

    let width = forest[0].len();
    let height = forest.len();

    print!(
        "{}",
        (0..width)
            .cartesian_product(0..height)
            .map(
                |(x, y)| (0..x)
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
                        .wrapping_add(1) //}
            )
            .max()
            .unwrap()
    );
}
