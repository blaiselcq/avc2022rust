use std::collections::HashSet;

fn no_duplicate(window: &str) -> bool {
    HashSet::<char>::from_iter(window.chars().into_iter()).len() == window.len()
}

fn get_start_parker_pos(string: &str, window_size: usize) -> Option<(usize, &str)> {
    if string.len() < window_size {
        return None;
    }
    for (j, _) in string.char_indices() {
        if j < window_size - 1 {
            continue;
        }
        let i = j - (window_size - 1);
        let start_seq_detected = no_duplicate(&string[i..=j]);
        if start_seq_detected {
            return Some((j, &string[j + 1..]));
        }
    }
    None
}

pub fn puzzle_1(input: &str) -> String {
    let pos = get_start_parker_pos(input.trim(), 4).unwrap().0 + 1;
    pos.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let pos = get_start_parker_pos(input.trim(), 14).unwrap().0 + 1;
    pos.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    use core::panic;

    use super::*;

    #[test]
    fn test_detect_start() {
        if let Some((i, string)) = get_start_parker_pos(&INPUT, 4) {
            assert_eq!(i + 1, 11);
            assert_eq!(string, "ljwzlrfnpqdbhtmscgvjw");
        } else {
            panic!()
        }
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(puzzle_1(&INPUT), "11");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(puzzle_2(&INPUT), "26");
    }
}
