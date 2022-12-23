use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn breadth_first_search<'a>(
    node: &'a str,
    rooms: &HashMap<&str, (usize, Vec<&'a str>)>,
    shortest_paths: &mut HashMap<&'a str, Vec<(&'a str, usize)>>,
) {
    let mut unvisited = VecDeque::from([(node, 0)]);
    let mut visited = HashSet::new();

    while let Some((child, steps)) = unvisited.pop_front() {
        for &neighbor in &rooms.get(child).unwrap().1 {
            if visited.contains(neighbor) {
                continue;
            }

            visited.insert(neighbor);
            unvisited.push_back((neighbor, steps + 1));

            if rooms.get(neighbor).unwrap().0 > 0 {
                shortest_paths
                    .get_mut(node)
                    .unwrap()
                    .push((neighbor, steps + 2));
            }
        }
    }
}

fn main() {
    let mut rooms = HashMap::new();

    let valve_regex = Regex::new(r"[A-Z]{2}").unwrap();
    let flow_regex = Regex::new(r"\d+").unwrap();

    for line in include_str!("input.txt").lines() {
        let valves = valve_regex
            .find_iter(line)
            .map(|valve| valve.as_str())
            .collect_vec();

        let flow = flow_regex
            .find(line)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        rooms.insert(valves[0], (flow, valves[1..].to_vec()));
    }

    let mut shortest_paths = HashMap::new();

    for (&node, (flow, _)) in &rooms {
        if node == "AA" || *flow > 0 {
            shortest_paths.insert(node, vec![]);
            breadth_first_search(node, &rooms, &mut shortest_paths);
        }
    }

    let mut max_flow = 0;

    for k in 1..=shortest_paths.len() / 2 {
        for santa_set in shortest_paths.keys().combinations(k) {
            let mut unvisited = vec![(vec!["AA"], 0, 26)];
            let mut best_heads = HashMap::new();
            let mut santa_flow = 0;

            while let Some((mut path, total, steps)) = unvisited.pop() {
                for &(node, distance) in shortest_paths.get(path.last().unwrap()).unwrap() {
                    if distance >= steps {
                        break;
                    }
                    if path.contains(&node) || santa_set.contains(&&node) {
                        continue;
                    }

                    let flow = total + rooms.get(node).unwrap().0 * (steps - distance);
                    santa_flow = santa_flow.max(flow);

                    if steps - distance >= 3 && flow > *best_heads.get(&(node, steps)).unwrap_or(&0)
                    {
                        path.push(node);
                        unvisited.push((path.clone(), flow, steps - distance));
                        path.pop();
                        best_heads.insert((node, steps - distance), flow);
                    }
                }
            }

            let mut unvisited = vec![(vec!["AA"], 0, 26)];
            let mut best_heads = HashMap::new();
            let mut elephant_flow = 0;

            while let Some((mut path, total, steps)) = unvisited.pop() {
                for &(node, distance) in shortest_paths.get(path.last().unwrap()).unwrap() {
                    if distance >= steps {
                        break;
                    }
                    if path.contains(&node) || !santa_set.contains(&&node) {
                        continue;
                    }

                    let flow = total + rooms.get(node).unwrap().0 * (steps - distance);
                    elephant_flow = elephant_flow.max(flow);

                    if steps - distance >= 3 && flow > *best_heads.get(&(node, steps)).unwrap_or(&0)
                    {
                        path.push(node);
                        unvisited.push((path.clone(), flow, steps - distance));
                        path.pop();
                        best_heads.insert((node, steps - distance), flow);
                    }
                }
            }

            max_flow = max_flow.max(santa_flow + elephant_flow);
        }
    }

    // let excluded = vec!["BB", "CC", "DD", "EE", "HH"];

    print!("{}", max_flow);
}
