use std::collections::VecDeque;

fn find_first_position(s: &str, window_size: usize) -> usize {
    let mut window = VecDeque::new();
    let mut rolling_hash = 0usize;

    for (i, c) in s.chars().map(|char| char as u8).enumerate() {
        window.push_back(c);
        rolling_hash ^= 1 << (c - b'a');
        if i >= window_size {
            let removed = window.pop_front().unwrap();
            rolling_hash ^= 1 << (removed - b'a');
            if rolling_hash.count_ones() as usize == window_size {
                return i + 1;
            }
        }
    }
    unreachable!()
}

pub fn p1(input: &str) -> String {
    find_first_position(input, 4).to_string()
}

pub fn p2(input: &str) -> String {
    find_first_position(input, 14).to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../../inputs/d06_example.txt")), "5");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d06.txt")), "1480");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../../inputs/d06_example.txt")), "23");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d06.txt")), "2746");
}
