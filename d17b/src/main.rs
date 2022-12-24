use std::collections::HashSet;

fn find_rocks_and_height(chars: usize, rocks: usize) -> (usize, usize) {
    let shapes = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // horizontal line
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // plus
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // L
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // vertical line
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],         // square
    ];

    let mut blocked_tiles = HashSet::new();
    for x in 0..7 {
        blocked_tiles.insert((x, 0));
    }

    let mut instructions = include_str!("input.txt").trim_end().chars().cycle();

    let mut dx;
    let mut fallen_rocks = 0;
    let mut char_count = 0;
    for shape in shapes.iter().cycle() {
        if fallen_rocks == rocks {
            return (blocked_tiles.iter().map(|(_, y)| *y).max().unwrap(), 0);
        }

        let mut x = 2;
        let mut y = blocked_tiles.iter().map(|(_, y)| y).max().unwrap() + 4;

        'next_shape: loop {
            if char_count == chars {
                return (
                    blocked_tiles.iter().map(|(_, y)| *y).max().unwrap(),
                    fallen_rocks,
                );
            }
            char_count += 1;

            match instructions.next().unwrap() {
                '>' => dx = 1,
                '<' => dx = -1,
                _ => unreachable!(),
            };

            for (nx, ny) in shape {
                if blocked_tiles.contains(&(x + nx + dx, y + ny))
                    || !(0..7).contains(&(x + nx + dx))
                {
                    dx = 0;
                }
            }

            x += dx;

            for (nx, ny) in shape {
                if blocked_tiles.contains(&(x + nx, y + ny - 1)) {
                    fallen_rocks += 1;
                    for (nx, ny) in shape {
                        blocked_tiles.insert((x + nx, y + ny));
                    }
                    break 'next_shape;
                }
            }

            y -= 1;
        }
    }
    unreachable!();
}

fn main() {
    let (s_height, s_rocks) =
        find_rocks_and_height(include_str!("input.txt").trim_end().len() * 5, usize::MAX);
    let (t_height, t_rocks) = find_rocks_and_height(
        include_str!("input.txt").trim_end().len() * 5 * 2,
        usize::MAX,
    );
    let delta_height = t_height - s_height;
    let delta_rocks = t_rocks - s_rocks;
    let (offset, _) = find_rocks_and_height(usize::MAX, 1000000000000 % delta_rocks);
    println!("{:?}", 1000000000000 / delta_rocks * delta_height + offset);
}
