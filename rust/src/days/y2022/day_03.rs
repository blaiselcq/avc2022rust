use std::collections::HashSet;

const PRIORITIES: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priorities(c: char) -> usize {
    PRIORITIES.find(c).unwrap()
}

fn get_shared_letter(first: &str, second: &str) -> char {
    let char_set_first: HashSet<char> = HashSet::from_iter(first.chars());
    let char_set_second: HashSet<char> = HashSet::from_iter(second.chars());

    let mut inter = char_set_first.intersection(&char_set_second);
    *inter.next().unwrap()
}

fn get_shared_letters(input: &str) -> Vec<char> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|letters| {
            let l = letters.len();
            let (first, second) = letters.split_at(l / 2);
            get_shared_letter(first, second)
        })
        .collect()
}

fn get_group_shared_letter(chunk: &[&str]) -> Option<char> {
    for letter in chunk.first().unwrap().chars() {
        let in_all_bags = chunk.iter().all(|letters| letters.contains(letter));
        if in_all_bags {
            return Some(letter);
        }
    }
    None
}

fn get_group_letters(input: &str) -> Vec<char> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| get_group_shared_letter(chunk).unwrap())
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let shared_letters = get_shared_letters(input);

    let sum: usize = shared_letters.iter().map(|x| get_priorities(*x)).sum();
    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let group_letters = get_group_letters(input);

    let sum: usize = group_letters.iter().map(|x| get_priorities(*x)).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_priorities() {
        assert_eq!(get_priorities('p'), 16);
        assert_eq!(get_priorities('L'), 38);
    }

    #[test]
    fn test_get_shared_letter() {
        assert_eq!(get_shared_letter("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p');
    }

    #[test]
    fn test_get_shared_letters() {
        let input = utils::get_input(utils::InputKind::Test, 22, 3).unwrap();
        assert_eq!(
            get_shared_letters(&input),
            vec!['p', 'L', 'P', 'v', 't', 's']
        );
    }

    #[test]
    fn test_puzzle_1() {
        let input = utils::get_input(utils::InputKind::Test, 22, 3).unwrap();
        let result = puzzle_1(&input);

        assert_eq!(result, "157");
    }

    #[test]
    fn test_puzzle_2() {
        let input = utils::get_input(utils::InputKind::Test, 22, 3).unwrap();
        let result = puzzle_2(&input);

        assert_eq!(result, "70");
    }
}
