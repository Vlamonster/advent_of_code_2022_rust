use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .windows(4)
            .position(|window| HashSet::<&u8>::from_iter(window).len() == 4)
            .unwrap()
            + 4
    );
}
