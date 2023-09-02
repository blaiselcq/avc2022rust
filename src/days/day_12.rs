use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq)]
struct TreeNode {
    position: Coords,
    path: Vec<Coords>,
    children: Vec<TreeNode>,
}

impl Coords {
    fn shift_left(self) -> Self {
        Self {
            x: self.x.saturating_sub(1),
            y: self.y,
        }
    }
    fn shift_up(self) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_sub(1),
        }
    }
    fn shift_right(self) -> Self {
        Self {
            x: self.x.saturating_add(1),
            y: self.y,
        }
    }
    fn shift_down(self) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_add(1),
        }
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.path
                .len()
                .cmp(&other.path.len())
                .then(self.position.cmp(&other.position)),
        )
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

fn parse_input(input: &str) -> (Coords, Coords, BTreeMap<Coords, char>) {
    let mut height_map_char: BTreeMap<Coords, char> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map({
                move |(x, h)| {
                    let coords = Coords { x, y };
                    (coords, h)
                }
            })
        })
        .collect();

    let &start = height_map_char.iter().find(|&(_, h)| *h == 'S').unwrap().0;
    let &goal = height_map_char.iter().find(|&(_, h)| *h == 'E').unwrap().0;

    *height_map_char.get_mut(&start).unwrap() = 'a';
    *height_map_char.get_mut(&goal).unwrap() = 'z';

    (start, goal, height_map_char)
}

fn find_accesible_squares(
    height_map: &BTreeMap<Coords, char>,
    current_position: Coords,
) -> Vec<Coords> {
    let mut accessible_squares = vec![];

    let &current_height = height_map.get(&current_position).unwrap();

    for potential_position in [
        current_position.shift_left(),
        current_position.shift_up(),
        current_position.shift_right(),
        current_position.shift_down(),
    ] {
        if let Some(&height) = height_map.get(&potential_position) {
            let height_diff = height as i8 - current_height as i8;
            if height_diff < 2 {
                accessible_squares.push(potential_position);
            }
        }
    }

    accessible_squares
}

fn find_solution(
    height_map: BTreeMap<Coords, char>,
    start: Option<Coords>,
    end: Coords,
) -> Vec<Coords> {
    let mut visited_positions = BTreeMap::new();

    // The ordering uses position and path, which are not modified
    #[allow(clippy::mutable_key_type)]
    let mut queue = BTreeSet::new();

    let mut start_condidates = match start {
        Some(start) => vec![TreeNode {
            position: start,
            path: vec![],
            children: vec![],
        }],
        None => height_map
            .iter()
            .filter(|&(_, c)| *c == 'a')
            .map(|(position, _)| TreeNode {
                position: *position,
                path: vec![],
                children: vec![],
            })
            .collect(),
    };

    for node in &mut start_condidates {
        queue.insert(node);
    }

    let mut results = vec![];

    while let Some(node) = queue.pop_first() {
        let accessible_positions = find_accesible_squares(&height_map, node.position);

        for position in accessible_positions {
            if position == node.position {
                continue;
            }

            let mut path = node.path.clone();
            path.push(position);

            if let Some(path_length) = visited_positions.get_mut(&position) {
                if *path_length <= path.len() {
                    continue;
                } else {
                    *path_length = path.len();
                }
            } else {
                visited_positions.insert(position, path.len());
            }

            let child = TreeNode {
                position,
                path,
                children: vec![],
            };

            if position == end {
                results.push(child);
                break;
            }
            node.children.push(child);
        }
        for child in &mut node.children {
            queue.insert(child);
        }
    }

    results.sort_by_key(|node| node.path.len());
    if let Some(path) = results.first() {
        return path.path.clone();
    }
    vec![]
}

pub fn puzzle_1(input: &str) -> String {
    let (start, goal, height_map) = parse_input(input);
    let solution = find_solution(height_map, Some(start), goal);

    solution.len().to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let (_, goal, height_map) = parse_input(input);
    let solution = find_solution(height_map, None, goal);

    solution.len().to_string()
}

#[cfg(test)]
mod tests {

    fn get_input() -> String {
        let day = 12;
        let input_file_path = format!("./data/tests/test{:02}.txt", day);
        std::fs::read_to_string(input_file_path).unwrap()
    }

    use super::*;

    #[test]
    fn test_can_parse_input() {
        let input = "Sab\nabc\nacE\n";

        let (start, goal, height_map) = parse_input(input);
        assert_eq!(start, Coords { x: 0, y: 0 });
        assert_eq!(goal, Coords { x: 2, y: 2 });

        assert_eq!(height_map[&goal], 'z');
        assert_eq!(height_map[&Coords { x: 1, y: 1 }], 'b');
    }

    #[test]
    fn test_find_accessible_squares() {
        let input = "Sab\nabc\nacE\n";

        let (_, _, height_map) = parse_input(input);

        let accessible_positions = find_accesible_squares(&height_map, Coords { x: 2, y: 1 });
        assert_eq!(
            accessible_positions,
            vec![Coords { x: 1, y: 1 }, Coords { x: 2, y: 0 }]
        );
    }

    #[test]
    fn test_puzzle_1() {
        let input = get_input();
        let result = puzzle_1(&input);

        assert_eq!(result, "31");
    }

    #[test]
    fn test_puzzle_2() {
        let input = get_input();
        let result = puzzle_2(&input);

        assert_eq!(result, "29");
    }
}
