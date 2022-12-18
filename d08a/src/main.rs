use std::cmp::max;

fn main() {
    let forest = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().map(|y| y as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let forest_width = forest[0].len();
    let forest_height = forest.len();

    let mut top_view: Vec<Vec<u8>> = vec![vec![0; forest_width]; forest_height];
    let mut bottom_view: Vec<Vec<u8>> = vec![vec![0; forest_width]; forest_height];
    let mut left_view: Vec<Vec<u8>> = vec![vec![0; forest_width]; forest_height];
    let mut right_view: Vec<Vec<u8>> = vec![vec![0; forest_width]; forest_height];

    for y in 0..forest_height {
        for x in 0..forest_width {
            if y == 0 {
                top_view[y][x] = forest[y][x];
            } else {
                top_view[y][x] = max(forest[y][x], top_view[y - 1][x]);
            }
        }
    }
    for y in (0..forest_height).rev() {
        for x in 0..forest_width {
            if y == forest_height - 1 {
                bottom_view[y][x] = forest[y][x];
            } else {
                bottom_view[y][x] = max(forest[y][x], bottom_view[y + 1][x]);
            }
        }
    }
    for x in 0..forest_width {
        for y in 0..forest_height {
            if x == 0 {
                left_view[y][x] = forest[y][x];
            } else {
                left_view[y][x] = max(forest[y][x], left_view[y][x - 1]);
            }
        }
    }
    for x in (0..forest_width).rev() {
        for y in 0..forest_height {
            if x == forest_width - 1 {
                right_view[y][x] = forest[y][x];
            } else {
                right_view[y][x] = max(forest[y][x], right_view[y][x + 1]);
            }
        }
    }

    let mut visible_trees = 0;
    for y in 0..forest_height {
        for x in 0..forest_width {
            if y == 0 || y == forest_height - 1 || x == 0 || x == forest_width - 1 {
                visible_trees += 1;
            } else if forest[y][x] > top_view[y - 1][x]
                || forest[y][x] > bottom_view[y + 1][x]
                || forest[y][x] > left_view[y][x - 1]
                || forest[y][x] > right_view[y][x + 1]
            {
                visible_trees += 1;
            }
        }
    }

    print!("{}", visible_trees);
}
