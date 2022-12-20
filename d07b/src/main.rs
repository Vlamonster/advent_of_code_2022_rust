use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let mut file_system = HashMap::new();
    let mut current_path = PathBuf::from("/");
    file_system.insert(current_path.clone(), 0usize);

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
                file_system.insert(current_path.clone(), 0);
            }
            ["dir", _] => {}
            ["$", "ls"] => {}
            [file_size, _] => {
                for path in current_path.ancestors() {
                    *file_system.get_mut(path).unwrap() += file_size.parse::<usize>().unwrap();
                }
            }
            _ => {}
        };
    }

    print!(
        "{}",
        file_system
            .values()
            .filter(|size| **size >= *file_system.values().max().unwrap() - 40000000)
            .min()
            .unwrap()
    );
}
