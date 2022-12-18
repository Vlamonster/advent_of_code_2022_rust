fn main() {
    let forest = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().map(|y| y as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let forest_width = forest[0].len();
    let forest_height = forest.len();

    let mut best_score = 0;

    for y in 0..forest_height {
        for x in 0..forest_width {
            let mut top_score = 0;
            let mut bottom_score = 0;
            let mut left_score = 0;
            let mut right_score = 0;

            for i in (0..y).rev() {
                top_score += 1;
                if forest[y][x] <= forest[i][x] {
                    break;
                }
            }
            for i in y + 1..forest_height {
                bottom_score += 1;
                if forest[y][x] <= forest[i][x] {
                    break;
                }
            }
            for i in (0..x).rev() {
                left_score += 1;
                if forest[y][x] <= forest[y][i] {
                    break;
                }
            }
            for i in x + 1..forest_width {
                right_score += 1;
                if forest[y][x] <= forest[y][i] {
                    break;
                }
            }

            let score = top_score * bottom_score * left_score * right_score;
            if score > best_score {
                best_score = score;
            }
        }
    }

    print! {"{}", best_score};
}
