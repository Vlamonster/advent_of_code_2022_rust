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

        cubes.insert((x, y, z));
    }

    let x_min = cubes.iter().map(|(x, _, _)| x).min().unwrap();
    let x_max = cubes.iter().map(|(x, _, _)| x).max().unwrap();
    let y_min = cubes.iter().map(|(_, y, _)| y).min().unwrap();
    let y_max = cubes.iter().map(|(_, y, _)| y).max().unwrap();
    let z_min = cubes.iter().map(|(_, _, z)| z).min().unwrap();
    let z_max = cubes.iter().map(|(_, _, z)| z).max().unwrap();

    let mut unvisited = vec![(x_min - 1, y_min - 1, z_min - 1)];
    let mut visited = HashSet::new();

    while let Some((x, y, z)) = unvisited.pop() {
        visited.insert((x, y, z));
        for (dx, dy, dz) in NEIGHBORS {
            if cubes.contains(&(x + dx, y + dy, z + dz)) {
                surface_area += 1;
            } else if !visited.contains(&(x + dx, y + dy, z + dz))
                && !unvisited.contains(&(x + dx, y + dy, z + dz))
                && (x_min - 1..=x_max + 1).contains(&(x + dx))
                && (y_min - 1..=y_max + 1).contains(&(y + dy))
                && (z_min - 1..=z_max + 1).contains(&(z + dz))
            {
                unvisited.push((x + dx, y + dy, z + dz));
            }
        }
    }
    print!("{}", surface_area);
}
