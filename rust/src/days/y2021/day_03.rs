use core::str;

use itertools::Itertools;

fn parse_input(input: &str) -> (u16, Vec<u16>) {
    let split = input.split('\n').filter(|l| !l.is_empty()).collect_vec();

    (
        split.first().unwrap().len() as u16,
        split
            .iter()
            .map(|l| u16::from_str_radix(l, 2).unwrap())
            .collect(),
    )
}

fn get_most_common_nth_bit(bits: &Vec<u16>, k: u16) -> u16 {
    let counts = bits.iter().map(|&n| (n & (1 << k)) >> k).counts();
    if counts.get(&0).unwrap_or(&0) > counts.get(&1).unwrap_or(&1) {
        0
    } else {
        1
    }
}

fn get_least_common_nth_bit(bits: &Vec<u16>, k: u16) -> u16 {
    get_most_common_nth_bit(bits, k) ^ 1
}

fn filter_on_most_common_nth_bit(bits: &Vec<u16>, k: u16) -> Vec<u16> {
    if bits.len() == 1 {
        return bits.clone();
    }

    let mcnb = get_most_common_nth_bit(bits, k);
    bits.iter()
        .filter(|&n| (n & (1 << k)) >> k == mcnb)
        .map(|&n| n)
        .collect_vec()
}

fn filter_on_least_common_nth_bit(bits: &Vec<u16>, k: u16) -> Vec<u16> {
    if bits.len() == 1 {
        return bits.clone();
    }

    let lcnb = get_least_common_nth_bit(bits, k);
    bits.iter()
        .filter(|&n| (n & (1 << k)) >> k == lcnb)
        .map(|&n| n)
        .collect_vec()
}

pub fn puzzle_1(input: &str) -> String {
    let (len, input) = parse_input(input);
    let gamma_rate = (0..len)
        .map(|i| get_most_common_nth_bit(&input, i) * 1 << i)
        .reduce(|acc, e| acc | e)
        .unwrap() as u32;
    let epsilon_rate = (0..len)
        .map(|i| get_least_common_nth_bit(&input, i) * 1 << i)
        .reduce(|acc, e| acc | e)
        .unwrap() as u32;

    (gamma_rate * epsilon_rate).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (len, input) = parse_input(input);

    let oo_rating = *(0..len)
        .rev()
        .fold(input.clone(), |acc, n| {
            filter_on_most_common_nth_bit(&acc, n)
        })
        .first()
        .unwrap() as u32;

    let coo_rating = *(0..len)
        .rev()
        .fold(input, |acc, n| filter_on_least_common_nth_bit(&acc, n))
        .first()
        .unwrap() as u32;

    (oo_rating * coo_rating).to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_puzzle_1() {
        let input = utils::get_input(utils::InputKind::Test, 21, 3).unwrap();
        let result = puzzle_1(&input);

        assert_eq!(result, "198");
    }
    #[test]
    fn test_puzzle_2() {
        let input = utils::get_input(utils::InputKind::Test, 21, 3).unwrap();
        let result = puzzle_2(&input);

        assert_eq!(result, "230");
    }
}
