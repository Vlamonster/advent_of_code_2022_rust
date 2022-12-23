use std::collections::HashSet;

fn main() {
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

    let mut instructions = include_str!("input.txt")
        .chars()
        .filter(|&char| char != '\n')
        .cycle();

    let mut dx;
    let mut fallen_rocks = 0;
    for shape in shapes.iter().cycle() {
        if fallen_rocks == 2022 {
            break;
        }

        let mut x = 2;
        let mut y = blocked_tiles.iter().map(|(_, y)| y).max().unwrap() + 4;

        'next_shape: loop {
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

    println!("{:?}", blocked_tiles.iter().map(|(_, y)| y).max().unwrap());
}
