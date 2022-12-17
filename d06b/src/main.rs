use std::collections::HashSet;

fn main() {
    print!(
        "{}",
        include_bytes!("input.txt")
            .windows(14)
            .position(|window| HashSet::<&u8>::from_iter(window).len() == 14)
            .unwrap()
            + 14
    );
}
