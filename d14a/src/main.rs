use std::collections::HashSet;

fn main() {
    let mut blocked_tiles = HashSet::new();
    for line in include_str!("input.txt").lines() {
        let mut path = Vec::new();
        for coordinates in line.split(" -> ") {
            let (x, y) = coordinates.split_once(',').unwrap();
            let x = x.parse::<isize>().unwrap();
            let y = y.parse::<isize>().unwrap();

            path.push((x, y));
        }
        let mut path_iter = path.windows(2);
        while let Some([(sx, sy), (tx, ty)]) = path_iter.next() {
            for nx in *sx.min(tx)..=*sx.max(tx) {
                for ny in *sy.min(ty)..=*sy.max(ty) {
                    blocked_tiles.insert((nx, ny));
                }
            }
        }
    }

    let min_y = *blocked_tiles.iter().map(|(_, y)| y).max().unwrap();
    let mut sand_piles = 0;

    loop {
        let (mut sand_x, mut sand_y) = (500, 0);

        loop {
            if sand_y > min_y {
                print!("{}", sand_piles);
                return;
            }
            if !blocked_tiles.contains(&(sand_x, sand_y + 1)) {
                sand_y += 1;
                continue;
            }
            if !blocked_tiles.contains(&(sand_x - 1, sand_y + 1)) {
                sand_x -= 1;
                sand_y += 1;
                continue;
            }
            if !blocked_tiles.contains(&(sand_x + 1, sand_y + 1)) {
                sand_x += 1;
                sand_y += 1;
                continue;
            }
            sand_piles += 1;
            blocked_tiles.insert((sand_x, sand_y));
            break;
        }
    }
}
