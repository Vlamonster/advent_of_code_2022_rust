use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn main() {
    let mut file_system = HashMap::new();
    let mut current_path = PathBuf::from("/");
    file_system.insert(current_path.clone(), (0, HashSet::<PathBuf>::new()));

    for terminal_line in include_str!("input.txt").lines() {
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

    print!(
        "{}",
        file_system
            .values()
            .map(|(size, _)| size)
            .filter(|size| **size
                >= *file_system.values().map(|(size, _)| size).max().unwrap() - 40000000)
            .min()
            .unwrap()
    );
}
