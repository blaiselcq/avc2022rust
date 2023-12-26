pub fn puzzle_1(input: &str) -> u32 {}

pub fn puzzle_2(input: &str) -> u32 {}

#[cfg(test)]
mod tests {

    fn get_input() -> String {
        let day = 0;
        let input_file_path = format!("../data/tests/test{:02}.txt", day);
        std::fs::read_to_string(input_file_path).unwrap()
    }

    use super::*;

    #[test]
    fn test_puzzle_1() {}

    #[test]
    fn test_puzzle_2() {}
}
