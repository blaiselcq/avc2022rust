use std::collections::BTreeSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListElement {
    Number(u8),
    List(Vec<ListElement>),
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ListElement::Number(a), ListElement::Number(b)) => a.partial_cmp(b),
            (ListElement::List(list_a), ListElement::List(list_b)) => list_a.partial_cmp(list_b),
            (ListElement::Number(a), ListElement::List(list_b)) => {
                vec![ListElement::Number(*a)].partial_cmp(list_b)
            }
            (ListElement::List(list_a), ListElement::Number(b)) => {
                list_a.partial_cmp(&vec![ListElement::Number(*b)])
            }
        }
    }
}

fn parse_input_list(input: &str) -> IResult<&str, ListElement> {
    let (input, result) = delimited(
        tag("["),
        separated_list0(
            tag(","),
            alt((map(complete::u8, ListElement::Number), parse_input_list)),
        ),
        tag("]"),
    )(input)?;

    Ok((input, ListElement::List(result)))
}

fn parse_input(input: &str) -> Vec<(ListElement, ListElement)> {
    let (_, result) = separated_list1(
        pair(newline, newline),
        separated_pair(parse_input_list, newline, parse_input_list),
    )(input)
    .unwrap();

    result
}

pub fn puzzle_1(input: &str) -> String {
    let pairs = parse_input(input);

    let sum: usize = pairs
        .iter()
        .map(|(left, right)| left <= right)
        .enumerate()
        .filter_map(|(id, result)| match result {
            true => Some(id + 1),
            false => None,
        })
        .sum();

    sum.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let input = parse_input(input);
    let mut packets = input
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<BTreeSet<_>>();
    let (_, divider_1) = parse_input_list("[[2]]").unwrap();
    let (_, divider_2) = parse_input_list("[[6]]").unwrap();

    packets.insert(&divider_1);
    packets.insert(&divider_2);

    let index_1 = packets.iter().position(|&el| el == &divider_1).unwrap() + 1;
    let index_2 = packets.iter().position(|&el| el == &divider_2).unwrap() + 1;

    (index_1 * index_2).to_string()
}

#[cfg(test)]
mod tests {

    fn get_input() -> String {
        let day = 13;
        let input_file_path = format!("./data/tests/test{:02}.txt", day);
        std::fs::read_to_string(input_file_path).unwrap()
    }

    use super::*;

    #[test]
    fn test_can_parse_list() {
        use ListElement::*;

        let input = "[[4,4],4,4]";
        let (_, result) = parse_input_list(input).unwrap();
        assert_eq!(
            result,
            List(vec![List(vec![Number(4), Number(4)]), Number(4), Number(4)])
        );

        let input = "[]";
        let (_, result) = parse_input_list(input).unwrap();
        assert_eq!(result, List(vec![]));
    }

    #[test]
    fn test_can_parse_input() {
        let input = get_input();
        let result = parse_input(&input);

        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_orderings() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a < b);

        let input = "[[1],[2,3,4]]\n[[1],4]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a < b);

        let input = "[9]\n[[8,7,6]]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a > b);

        let input = "[[4,4],4,4]\n[[4,4],4,4,4]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a < b);

        let input = "[7,7,7,7]\n[7,7,7]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a > b);

        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let (a, b) = parse_input(input).first().unwrap().clone();

        assert!(a > b);
    }

    #[test]
    fn test_puzzle_1() {
        let input = get_input();
        assert_eq!(puzzle_1(&input), "13");
    }

    #[test]
    fn test_puzzle_2() {
        let input = get_input();
        assert_eq!(puzzle_2(&input), "140");
    }
}
