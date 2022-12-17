fn score(game: &str) -> u8 {
    let opponent = game.chars().nth(0).unwrap();
    let you = game.chars().nth(2).unwrap();

    match you {
        'X' => {
            1 + match opponent {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => unreachable!(),
            }
        }
        'Y' => {
            2 + match opponent {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => unreachable!(),
            }
        }
        'Z' => {
            3 + match opponent {
                'A' => 0,
                'B' => 6,
                'C' => 3,
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
