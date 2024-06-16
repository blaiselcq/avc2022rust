use core::str;

fn load_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|d| str::parse(d).unwrap())
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let number = load_input(input)
        .windows(2)
        .map(|slice| slice[1] > slice[0])
        .filter(|&b| b)
        .count();

    number.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let sums = load_input(input)
        .windows(3)
        .map(|slice| slice.iter().sum())
        .collect::<Vec<u32>>();
    let number = sums
        .windows(2)
        .map(|slice| slice[1] > slice[0])
        .filter(|&b| b)
        .count();

    number.to_string()
}
