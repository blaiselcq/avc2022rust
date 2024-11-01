type Grid = [[u8; 5]; 5];

fn parse_numbers(line: &str) -> Vec<u8> {
    line.split(',')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_grid(grid: &str) -> Grid {
    grid.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Grid>) {
    let mut split = input.split("\n\n");
    let numbers = parse_numbers(split.next().unwrap());
    let grid = split
        .filter(|s| !s.is_empty())
        .map(|lines| parse_grid(lines))
        .collect();

    (numbers, grid)
}

fn is_winning(grid: &Grid) -> bool {
    for l in 0..5 {
        let row_wins = grid[l].iter().all(|&n| n == 0);
        let col_win = (0..5).all(|c| grid[c][l] == 0);
        if row_wins || col_win {
            return true;
        }
    }
    return false;
}

fn replace_number(grid: &mut Grid, number: u8) {
    for l in 0..5 {
        for c in 0..5 {
            if grid[l][c] == number {
                grid[l][c] = 0;
            }
        }
    }
}

fn get_grid_score(grid: &Grid, number: u32) -> u32 {
    number
        * (grid
            .iter()
            .map(|l| l.map(|n| n as u32).iter().sum::<u32>())
            .sum::<u32>())
}

pub fn puzzle_1(input: &str) -> String {
    let (numbers, mut grids) = parse_input(input);

    let mut winning_grid: Option<(u8, Grid)> = None;
    'o: for n in numbers {
        for grid in grids.iter_mut() {
            replace_number(grid, n);
            if is_winning(grid) {
                winning_grid = Some((n, grid.clone()));
                break 'o;
            }
        }
    }

    let (n, grid) = winning_grid.unwrap();
    get_grid_score(&grid, n as u32).to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (numbers, mut grids) = parse_input(input);

    let mut winning_grid: Option<(u8, Grid)> = None;
    for n in numbers {
        for grid in grids.iter_mut() {
            if is_winning(grid) {
                continue;
            }
            replace_number(grid, n);
            if is_winning(grid) {
                winning_grid = Some((n, grid.clone()));
            }
        }
    }

    let (n, grid) = winning_grid.unwrap();
    get_grid_score(&grid, n as u32).to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_puzzle_1() {
        let input = utils::get_input(utils::InputKind::Test, 21, 4).unwrap();
        assert_eq!(puzzle_1(&input), "4512");
    }
    #[test]
    fn test_puzzle_2() {
        let input = utils::get_input(utils::InputKind::Test, 21, 4).unwrap();
        assert_eq!(puzzle_2(&input), "1924");
    }
}
