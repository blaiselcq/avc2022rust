mod types {
    use std::collections::VecDeque;

    pub type Operation = dyn Fn(u64) -> Option<u64>;
    pub struct Monkey {
        pub items: VecDeque<u64>,
        pub operation: Box<Operation>,
        pub test_number: u64,
        pub recipient_success: usize,
        pub recipient_failed: usize,
    }

    impl Monkey {
        pub fn play(&mut self, managed_worry: bool, divisor: u64) -> Option<(usize, u64)> {
            let value = self.items.pop_front()?;

            let worry_level = match managed_worry {
                false => (self.operation)(value % divisor)?,
                true => (self.operation)(value)? / 3,
            };

            match worry_level % self.test_number == 0 {
                true => Some((self.recipient_success, worry_level)),
                false => Some((self.recipient_failed, worry_level)),
            }
        }

        pub fn throw(&mut self, value: u64) {
            self.items.push_back(value);
        }
    }
}

mod parser {
    use core::panic;
    use std::num::ParseIntError;

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::{self, anychar, digit1, multispace0, multispace1, newline},
        multi::separated_list0,
        sequence::{delimited, preceded, tuple},
        IResult,
    };

    pub use super::types::Monkey;

    #[derive(Debug, Clone, Copy)]
    enum Operand {
        Old,
        Number(u64),
    }

    fn map_operand(operand: &str) -> Result<Operand, ParseIntError> {
        match operand {
            "old" => Ok(Operand::Old),
            string => {
                let number = string.parse()?;
                Ok(Operand::Number(number))
            }
        }
    }

    fn parse_operation(input: &str) -> IResult<&str, Box<super::types::Operation>> {
        let (input, _) = tag("new = ")(input)?;
        let (input, operand_1_str) = alt((tag("old"), digit1))(input)?;
        let (input, operator_str) = delimited(multispace0, anychar, multispace0)(input)?;
        let (input, operand_2_str) = alt((tag("old"), digit1))(input)?;

        let operand_1 = map_operand(operand_1_str);
        let operand_2 = map_operand(operand_2_str);

        if operand_1.is_err() || operand_2.is_err() {
            return Err(nom::Err::Failure(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Digit,
            }));
        }

        let operand_1 = operand_1.unwrap();
        let operand_2 = operand_2.unwrap();

        let operator = match operator_str {
            '+' => u64::checked_add,
            '-' => panic!(),
            '*' => u64::checked_mul,
            _ => panic!(),
        };

        let operation: Box<dyn Fn(u64) -> Option<u64>> = match (operand_1, operand_2) {
            (Operand::Old, Operand::Old) => Box::new(move |old| operator(old, old)),
            (Operand::Old, Operand::Number(n2)) => Box::new(move |old| operator(old, n2)),
            (Operand::Number(n1), Operand::Old) => Box::new(move |old| operator(n1, old)),
            (Operand::Number(n1), Operand::Number(n2)) => Box::new(move |_| operator(n1, n2)),
        };

        Ok((input, operation))
    }

    fn parse_test(input: &str) -> IResult<&str, (u64, usize, usize)> {
        let (input, divisor) = preceded(tag("divisible by "), complete::u64)(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = take_until(": ")(input)?;
        let (input, monkey_1) = preceded(tag(": throw to monkey "), complete::u8)(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = take_until(": ")(input)?;
        let (input, monkey_2) = preceded(tag(": throw to monkey "), complete::u8)(input)?;

        Ok((input, (divisor, monkey_1.into(), monkey_2.into())))
    }

    fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
        let (input, _) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
        let (input, _) = newline(input)?;
        let (input, items) = preceded(
            tuple((multispace1, tag("Starting items: "))),
            separated_list0(tag(", "), complete::u64),
        )(input)?;
        let (input, _) = newline(input)?;
        let (input, operation) =
            preceded(tuple((multispace1, tag("Operation: "))), parse_operation)(input)?;
        let (input, _) = newline(input)?;
        let (input, (test_number, recipient_success, recipient_failed)) =
            preceded(tuple((multispace1, tag("Test: "))), parse_test)(input)?;

        Ok((
            input,
            Monkey {
                items: items.into(),
                operation,
                test_number,
                recipient_success,
                recipient_failed,
            },
        ))
    }

    pub fn parse_input(input: &str) -> Vec<Monkey> {
        let (_, monkeys) = separated_list0(multispace1, parse_monkey)(input).unwrap();
        monkeys
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

        let (_, monkey) = parse_monkey(input).unwrap();

        assert_eq!(monkey.items, vec![54, 65, 75, 74]);
        assert_eq!(monkey.test_number, 19);
        assert_eq!(monkey.recipient_success, 2);
        assert_eq!(monkey.recipient_failed, 0);
    }
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn monkey_play(
    iterations: usize,
    mut monkeys: Vec<types::Monkey>,
    managed_worry: bool,
) -> Vec<usize> {
    let mut pass = vec![0; monkeys.len()];

    let divisor = match managed_worry {
        true => 3,
        false => lcm(&monkeys.iter().map(|m| m.test_number).collect::<Vec<_>>()),
    };

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            while let Some(play) = monkeys[i].play(managed_worry, divisor) {
                let (recipient, value) = play;
                monkeys[recipient].throw(value);
                pass[i] += 1;
            }
        }
    }

    pass
}

pub fn puzzle_1(input: &str) -> String {
    let monkeys = parser::parse_input(input);
    let mut pass = monkey_play(20, monkeys, true);
    pass.sort();

    (pass.pop().unwrap() * pass.pop().unwrap()).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let monkeys = parser::parse_input(input);
    let mut pass = monkey_play(10000, monkeys, false);

    pass.sort();

    (pass.pop().unwrap() * pass.pop().unwrap()).to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_puzzle_1() {
        let input = utils::get_input(utils::InputKind::Test, 22, 11).unwrap();
        assert_eq!(puzzle_1(&input), "10605");
    }

    #[test]
    fn test_puzzle_2() {
        let input = utils::get_input(utils::InputKind::Test, 22, 11).unwrap();
        assert_eq!(puzzle_2(&input), "2713310158");
    }
}
