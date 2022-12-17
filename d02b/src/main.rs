fn score(game: &str) -> u8 {
    let opponent = game.chars().nth(0).unwrap();
    let you = game.chars().nth(2).unwrap();

    match you {
        'X' => {
            0 + match opponent {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => unreachable!(),
            }
        }
        'Y' => {
            3 + match opponent {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => unreachable!(),
            }
        }
        'Z' => {
            6 + match opponent {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|game| score(game) as usize)
            .sum::<usize>()
    );
}
