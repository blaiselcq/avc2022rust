use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Rope {
    knots: Vec<Pos>,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, ()> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            'L' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl Pos {
    fn move_direction(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Up => self.y += distance,
            Direction::Down => self.y -= distance,
            Direction::Right => self.x += distance,
            Direction::Left => self.x -= distance,
        };
    }
}

impl Rope {
    fn new(knots: usize, starting_pos: Pos) -> Rope {
        Rope {
            knots: vec![starting_pos; knots],
        }
    }
    fn get_tail(&self) -> Option<&Pos> {
        return self.knots.last();
    }
    fn move_head(&mut self, direction: Direction, distance: i32) -> Vec<Pos> {
        let mut tail_moves = vec![];

        for _ in 0..distance {
            if let Some(head) = self.knots.get_mut(0) {
                head.move_direction(direction, 1);
            } else {
                return tail_moves;
            }

            for id in 1..self.knots.len() {
                let last_knot = self.knots[id - 1];

                let dx = last_knot.x - self.knots[id].x;
                let dy = last_knot.y - self.knots[id].y;

                if dx.abs().max(dy.abs()) <= 1 {
                    continue;
                }

                self.knots[id].move_direction(Direction::Right, dx.signum());
                self.knots[id].move_direction(Direction::Up, dy.signum());
                if id == self.knots.len() - 1 {
                    tail_moves.push(self.knots[id]);
                }
            }
        }

        tail_moves
    }
}

#[allow(dead_code)]
fn print_debug_pos(input: &HashSet<Pos>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;

    input.iter().for_each(|Pos { x, y }| {
        if x < &min_x {
            min_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y > &max_y {
            max_y = *y;
        }
    });

    for y in (min_y..=max_y).rev() {
        let mut line = String::new();
        line.push_str(&format!("{}\t", y));
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                line.push('s');
            } else if input.contains(&Pos { x, y }) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{line}");
    }
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (direction, distance) = l.split_once(' ').unwrap();
            let direction: Option<Direction> =
                direction.chars().next().and_then(|c| c.try_into().ok());
            let distance: Option<i32> = distance.parse().ok();
            (direction, distance)
        })
        .filter_map(|(dir, dis)| match (dir, dis) {
            (Some(direction), Some(distance)) => Some((direction, distance)),
            (_, _) => None,
        })
        .collect()
}

fn get_tail_pos(mouvements: Vec<(Direction, i32)>, rope: &mut Rope) -> HashSet<Pos> {
    let mut positions = HashSet::new();
    positions.insert(*rope.get_tail().unwrap());

    for (direction, distance) in mouvements {
        for pos_tail in rope.move_head(direction, distance) {
            positions.insert(pos_tail);
        }
        // print_debug_pos(&positions);
    }

    positions
}

pub fn puzzle_1(input: &str) -> String {
    let mouvements = parse_input(input);

    let mut rope = Rope::new(2, Pos { x: 0, y: 0 });

    let positions = get_tail_pos(mouvements, &mut rope);
    positions.len().to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let mouvements = parse_input(input);

    let mut rope = Rope::new(10, Pos { x: 0, y: 0 });

    let positions = get_tail_pos(mouvements, &mut rope);
    positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_move_head_and_tail() {
        let mut rope = Rope {
            knots: vec![Pos { x: 3, y: 3 }, Pos { x: 2, y: 2 }],
        };
        let tail_pos = rope.move_head(Direction::Up, 1);
        assert_eq!(tail_pos.len(), 1);
        assert_eq!(rope.get_tail().unwrap().clone(), Pos { x: 3, y: 3 });
    }

    #[test]
    fn test_parse_input() {
        let input = "R 4\nU 4\n";
        assert_eq!(
            parse_input(input),
            vec![(Direction::Right, 4), (Direction::Up, 4)]
        );
    }

    #[test]
    fn test_puzzle_1() {
        let input = utils::get_input(utils::InputKind::Test, 22, 9).unwrap();

        assert_eq!(puzzle_1(&input), "88");
    }

    #[test]
    fn test_puzzle_2() {
        let input = utils::get_input(utils::InputKind::Test, 22, 9).unwrap();

        assert_eq!(puzzle_2(&input), "36");
    }
}
