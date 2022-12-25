use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn construct_file_system(input: &str) -> HashMap<PathBuf, (usize, HashSet<PathBuf>)> {
    let mut file_system = HashMap::new();
    let mut current_path = PathBuf::from("/");
    file_system.insert(current_path.clone(), (0, HashSet::<PathBuf>::new()));

    for terminal_line in input.lines() {
        match terminal_line.split_whitespace().collect_vec().as_slice() {
            ["$", "cd", "/"] => {
                current_path.push("/");
            }
            ["$", "cd", ".."] => {
                current_path.pop();
            }
            ["$", "cd", path] => {
                current_path.push(path);
                if !file_system.contains_key(&current_path) {
                    file_system.insert(current_path.clone(), (0, HashSet::new()));
                }
            }
            ["dir", _] => {}
            ["$", "ls"] => {}
            [file_size, file_name] => {
                if file_system
                    .get_mut(&current_path)
                    .unwrap()
                    .1
                    .insert(PathBuf::from(file_name))
                {
                    for path in current_path.ancestors() {
                        file_system.get_mut(path).unwrap().0 += file_size.parse::<usize>().unwrap();
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    file_system
}

pub fn p1(input: &str) -> String {
    construct_file_system(input)
        .values()
        .map(|(size, _)| size)
        .filter(|size| **size < 100000)
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &str) -> String {
    let file_system = construct_file_system(input);
    file_system
        .values()
        .map(|&(size, _)| size)
        .filter(|&size| {
            size >= file_system.values().map(|&(size, _)| size).max().unwrap() - 40000000
        })
        .min()
        .unwrap()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d07_example.txt")), "95437");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d07.txt")), "1423358");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d07_example.txt")), "24933642");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d07.txt")), "545729");
}
