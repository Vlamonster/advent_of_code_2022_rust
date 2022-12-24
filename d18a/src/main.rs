use itertools::Itertools;
use std::collections::HashSet;

const NEIGHBORS: [(isize, isize, isize); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn main() {
    let mut cubes = HashSet::new();
    let mut surface_area = 0;
    for line in include_str!("input.txt").lines() {
        let (x, y, z) = line
            .split(',')
            .map(|coordinate| coordinate.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        surface_area += 6;

        for (dx, dy, dz) in NEIGHBORS {
            if cubes.contains(&(x + dx, y + dy, z + dz)) {
                surface_area -= 2;
            }
        }

        cubes.insert((x, y, z));
    }

    print!("{}", surface_area);
}
