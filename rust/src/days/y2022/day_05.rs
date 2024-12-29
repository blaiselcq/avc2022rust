type Piles = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq)]
struct Step {
    number: u32,
    from: u32,
    to: u32,
}

enum Crane {
    Crane9000,
    Crane9001,
}

fn get_crates(line: &str, crate_number: usize) -> Vec<Option<char>> {
    let chunk_size = 4;

    let mut crates: Vec<Option<char>> = vec![None; crate_number];

    for (index, chunk) in line
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .enumerate()
    {
        let letter = chunk.iter().collect::<String>().trim().chars().nth(1);
        crates[index] = letter;
    }

    crates
}

fn parse_piles(input: &str) -> Piles {
    let mut lines = input.split('\n').rev();

    let number_line = lines.next().unwrap();
    let pile_numbers: Vec<u8> = number_line
        .chars()
        .filter_map(|c| c.to_string().parse::<u8>().ok())
        .collect();

    let crate_piles: Vec<_> = lines
        .filter(|line| !line.is_empty())
        .map(|line| get_crates(line, pile_numbers.len()))
        .collect();

    let mut piles: Piles = vec![Vec::new(); pile_numbers.len()];

    crate_piles.into_iter().for_each(|crates| {
        crates.into_iter().enumerate().for_each(|(i, c)| {
            if let Some(c) = c {
                piles[i].push(c)
            }
        })
    });

    piles
}

fn parse_steps(input: &str) -> Vec<Step> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (number_str, rest_str) = line.split_once("from").unwrap();
            let (from_str, to_str) = rest_str.split_once("to").unwrap();
            let number = number_str.replace("move", "").trim().parse().unwrap();
            let from = from_str.trim().parse().unwrap();
            let to = to_str.trim().parse().unwrap();
            Step { number, from, to }
        })
        .collect()
}

impl Step {
    fn perform_move(&self, piles: &mut Piles, crane_type: Crane) {
        for _ in 0..self.number {
            let pile_from = piles.get_mut(self.from as usize - 1).unwrap();
            let mut to_move = match crane_type {
                Crane::Crane9000 => vec![pile_from.pop().unwrap()],
                Crane::Crane9001 => pile_from.split_off(pile_from.len() - self.number as usize),
            };
            let pile_to = piles.get_mut(self.to as usize - 1).unwrap();
            match crane_type {
                Crane::Crane9000 => pile_to.push(*to_move.first().unwrap()),
                Crane::Crane9001 => {
                    pile_to.append(&mut to_move);
                    break;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (Piles, Vec<Step>) {
    let (piles, steps) = input.split_once("\n\n").unwrap();
    (parse_piles(piles), parse_steps(steps))
}

pub fn get_result(piles: &Piles) -> String {
    let mut result = String::new();
    piles.iter().for_each(|pile| {
        if let Some(c) = pile.last() {
            result.push(*c);
        }
    });

    result
}

pub fn puzzle_1(input: &str) -> String {
    let parsed = parse_input(input);
    let steps = parsed.1;
    let mut piles = parsed.0;

    steps
        .into_iter()
        .for_each(|step| step.perform_move(&mut piles, Crane::Crane9000));

    get_result(&piles)
}

pub fn puzzle_2(input: &str) -> String {
    let parsed = parse_input(input);
    let steps = parsed.1;
    let mut piles = parsed.0;

    steps
        .into_iter()
        .for_each(|step| step.perform_move(&mut piles, Crane::Crane9001));

    get_result(&piles)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    use super::*;

    #[test]
    fn test_get_crates() {
        let line = "[N] [C]    ";
        let crate_number = 3;
        let crates = get_crates(line, crate_number);
        assert_eq!(crates, vec![Some('N'), Some('C'), None]);
    }

    #[test]
    fn test_parse_piles() {
        let piles_str = "[A]    \n[C] [D]\n 1  2 ";
        let piles = parse_piles(piles_str);
        assert_eq!(piles, vec![vec!['C', 'A'], vec!['D']]);
    }

    #[test]
    fn test_parse_step() {
        let step_str = "move 3 from 1 to 3";
        let steps = parse_steps(step_str);
        assert_eq!(
            steps,
            vec![Step {
                number: 3,
                from: 1,
                to: 3
            }]
        );
    }

    #[test]
    fn test_puzzle_1() {
        let result = puzzle_1(INPUT);

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_puzzle_2() {
        let result = puzzle_2(INPUT);

        assert_eq!(result, "MCD");
    }
}
