use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut rooms = HashMap::new();

    let valve_regex = Regex::new(r"[A-Z]{2}").unwrap();
    let rate_regex = Regex::new(r"\d+").unwrap();

    for room in include_str!("input.txt").lines() {
        let valve = valve_regex.find(room).unwrap().as_str();

        let tunnels = valve_regex
            .find_iter(room)
            .skip(1)
            .map(|valve| valve.as_str())
            .collect_vec();

        let rate = rate_regex
            .find(room)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        rooms.insert(valve, (tunnels, rate));
    }

    let mut unvisited_paths = vec![(vec!["AA"], 30, 0)];
    let mut visited_paths = HashMap::new();

    while let Some((path, steps, total)) = unvisited_paths.pop() {
        visited_paths.insert(path.clone(), total);

        if steps <= 1 {
            continue;
        }

        let mut unvisited_rooms = VecDeque::from([(path.last().unwrap(), steps)]);
        let mut visited_rooms = HashSet::new();

        while let Some((unvisited_room, steps_inner)) = unvisited_rooms.pop_front() {
            if steps_inner <= 2 {
                break;
            }

            let (neighbors, _) = rooms.get(unvisited_room).unwrap();
            visited_rooms.insert(unvisited_room);

            for neighbor in neighbors {
                if !visited_rooms.contains(neighbor) {
                    unvisited_rooms.push_back((neighbor, steps_inner - 1));

                    if !path.contains(neighbor) {
                        let mut unvisited_path = path.clone();
                        unvisited_path.push(neighbor);
                        let (_, rate) = rooms.get(neighbor).unwrap();
                        if *rate > 0 {
                            unvisited_paths.push((
                                unvisited_path,
                                steps_inner - 2,
                                total + rate * (steps_inner - 2),
                            ))
                        }
                    }
                }
            }
        }
    }

    print!("{:?}", visited_paths.values().max().unwrap());
}
