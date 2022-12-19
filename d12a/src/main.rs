use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug)]
struct Graph {
    nodes: HashMap<(usize, usize), usize>,
    edges: HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Graph {
    pub fn insert_node(&mut self, x: usize, y: usize, value: usize) {
        self.nodes.insert((x, y), value);
        self.edges.insert((x, y), Vec::new());
    }
}

fn init_graph() -> Graph {
    let mut graph = Graph {
        nodes: Default::default(),
        edges: Default::default(),
        start: (0, 0),
        end: (0, 0),
    };

    let mut height_map = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|x| x as usize).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = height_map.len();
    let width = height_map[0].len();

    // insert nodes
    for y in 0..height {
        for x in 0..width {
            if height_map[y][x] == 'S' as usize {
                height_map[y][x] = 'a' as usize;
                graph.start = (x, y);
            } else if height_map[y][x] == 'E' as usize {
                height_map[y][x] = 'z' as usize;
                graph.end = (x, y);
            }
            graph.insert_node(x, y, height_map[y][x]);
        }
    }

    // insert edges
    for y in 0..height {
        for x in 0..width {
            // check above
            match graph.nodes.get(&(x, y.wrapping_sub(1))) {
                None => {}
                Some(node) => {
                    if *node <= graph.nodes.get(&(x, y)).unwrap() + 1 {
                        graph
                            .edges
                            .get_mut(&(x, y))
                            .unwrap()
                            .push((x, y.wrapping_sub(1)))
                    }
                }
            }
            // check below
            match graph.nodes.get(&(x, y.wrapping_add(1))) {
                None => {}
                Some(node) => {
                    if *node <= graph.nodes.get(&(x, y)).unwrap() + 1 {
                        graph
                            .edges
                            .get_mut(&(x, y))
                            .unwrap()
                            .push((x, y.wrapping_add(1)))
                    }
                }
            }
            // check to the left
            match graph.nodes.get(&(x.wrapping_sub(1), y)) {
                None => {}
                Some(node) => {
                    if *node <= graph.nodes.get(&(x, y)).unwrap() + 1 {
                        graph
                            .edges
                            .get_mut(&(x, y))
                            .unwrap()
                            .push((x.wrapping_sub(1), y))
                    }
                }
            }
            // check to the right
            match graph.nodes.get(&(x.wrapping_add(1), y)) {
                None => {}
                Some(node) => {
                    if *node <= graph.nodes.get(&(x, y)).unwrap() + 1 {
                        graph
                            .edges
                            .get_mut(&(x, y))
                            .unwrap()
                            .push((x.wrapping_add(1), y))
                    }
                }
            }
        }
    }

    graph
}

fn main() {
    let graph = init_graph();
    let mut paths: BTreeMap<(usize, usize), Option<Vec<(usize, usize)>>> = BTreeMap::new();
    let mut visited = BTreeSet::new();
    let mut unvisited = BTreeSet::new();
    unvisited.insert(graph.start);
    paths.insert(graph.start, Some(vec![]));

    while !unvisited.is_empty() {
        let current_node = unvisited.pop_first().unwrap();
        visited.insert(current_node);
        for neighbor in graph.edges.get(&current_node).unwrap() {
            match paths.get(neighbor) {
                None => {
                    let mut path = paths.get(&current_node).unwrap().clone().unwrap();
                    path.push(current_node);
                    paths.insert(*neighbor, Some(path));
                    unvisited.insert(*neighbor);
                }
                Some(current_path) => {
                    let mut path = paths.get(&current_node).unwrap().clone().unwrap();
                    path.push(current_node);
                    if path.len() < current_path.clone().unwrap().len() {
                        paths.insert(*neighbor, Some(path));
                        unvisited.insert(*neighbor);
                    }
                }
            }
        }
    }

    print!(
        "{}",
        paths.get(&graph.end).unwrap().clone().unwrap().len()
    );
}
