use core::str;
use std::error::Error;

enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct Coord {
    distance: u32,
    depth: u32,
    aim: u32,
}

fn parse_line(line: &str) -> Result<Movement, Box<dyn Error>> {
    let (direction, distance) = line.split_once(" ").ok_or("Cannot split line")?;
    let distance = str::parse(distance)?;
    match direction {
        "forward" => Ok(Movement::Forward(distance)),
        "down" => Ok(Movement::Down(distance)),
        "up" => Ok(Movement::Up(distance)),
        string => Err(format!("Unknown direction {string}"))?,
    }
}

fn load_input(input: &str) -> Vec<Movement> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line).unwrap())
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let coords = load_input(input).iter().fold(
        Coord {
            distance: 0,
            depth: 0,
            aim: 0,
        },
        |res, movement| match movement {
            Movement::Forward(d) => Coord {
                distance: res.distance + d,
                depth: res.depth,
                aim: res.aim,
            },
            Movement::Down(d) => Coord {
                distance: res.distance,
                depth: res.depth + d,
                aim: res.aim,
            },
            Movement::Up(d) => Coord {
                distance: res.distance,
                depth: res.depth - d,
                aim: res.aim,
            },
        },
    );

    { coords.distance * coords.depth }.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let coords = load_input(input).iter().fold(
        Coord {
            distance: 0,
            depth: 0,
            aim: 0,
        },
        |res, movement| match movement {
            Movement::Forward(d) => Coord {
                distance: res.distance + d,
                depth: res.depth + d * res.aim,
                aim: res.aim,
            },
            Movement::Down(d) => Coord {
                distance: res.distance,
                depth: res.depth,
                aim: res.aim + d,
            },
            Movement::Up(d) => Coord {
                distance: res.distance,
                depth: res.depth,
                aim: res.aim - d,
            },
        },
    );

    { coords.distance * coords.depth }.to_string()
}
