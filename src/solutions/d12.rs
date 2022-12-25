use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<u8>>;
type Node = (usize, usize);

fn breadth_first_search(area: Grid, starting_nodes: Vec<(Node, usize)>, end: Node) -> usize {
    let mut unvisited = VecDeque::from(starting_nodes);
    let mut visited = HashSet::new();
    while let Some(((x, y), steps)) = unvisited.pop_front() {
        if (x, y) == end {
            return steps;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if let Some(&neighbor) = area.get(ny).and_then(|row| row.get(nx)) {
                if area[y][x] + 1 >= neighbor && !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));
                    unvisited.push_back(((nx, ny), steps + 1));
                }
            } else {
                continue;
            }
        }
    }
    unreachable!()
}

fn get_area(input: &str) -> (Node, Node, Grid) {
    let mut area = input.lines().map(|x| x.as_bytes().to_vec()).collect_vec();

    let (sx, sy) = (0..area[0].len())
        .cartesian_product(0..area.len())
        .find(|&(x, y)| area[y][x] == b'S')
        .unwrap();
    let (tx, ty) = (0..area[0].len())
        .cartesian_product(0..area.len())
        .find(|&(x, y)| area[y][x] == b'E')
        .unwrap();

    area[sy][sx] = b'a';
    area[ty][tx] = b'z';

    ((sx, sy), (tx, ty), area)
}

pub fn p1(input: &str) -> String {
    let (start, end, area) = get_area(input);
    breadth_first_search(area, vec![(start, 0)], end).to_string()
}

pub fn p2(input: &str) -> String {
    let (_, end, area) = get_area(input);
    let starting_nodes = (0..area[0].len())
        .cartesian_product(0..area.len())
        .filter(|&(x, y)| area[y][x] == b'a')
        .map(|node| (node, 0))
        .collect_vec();
    breadth_first_search(area, starting_nodes, end).to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d12_example.txt")), "31");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d12.txt")), "520");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d12_example.txt")), "29");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d12.txt")), "508");
}
