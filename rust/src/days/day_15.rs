use std::{collections::BTreeMap, ops::RangeInclusive};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Coord {
    x: i32,
    y: i32,
}

pub(crate) struct Sensor {
    closest_beacon: Coord,
    distance: i32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum MapState {
    Unknown,
    Empty,
    Beacon,
    Sensor,
}

impl Coord {
    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y))
            .try_into()
            .unwrap()
    }
}

mod parser {
    use std::collections::BTreeMap;

    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        IResult,
    };

    use super::{Coord, Sensor};

    fn parse_coord(input: &str) -> IResult<&str, Coord> {
        let (input, coord) = separated_pair(
            preceded(tag("x="), complete::i32),
            tag(", "),
            preceded(tag("y="), complete::i32),
        )(input)?;
        let coord = Coord {
            x: coord.0,
            y: coord.1,
        };

        Ok((input, coord))
    }

    fn parse_line(input: &str) -> IResult<&str, (Coord, Sensor)> {
        let (input, (sensor, closest_beacon)) = separated_pair(
            preceded(tag("Sensor at "), parse_coord),
            tag(": "),
            preceded(tag("closest beacon is at "), parse_coord),
        )(input)?;

        let distance = sensor.manhattan_distance(&closest_beacon);
        Ok((
            input,
            (
                sensor,
                Sensor {
                    closest_beacon,
                    distance,
                },
            ),
        ))
    }

    pub(crate) fn parse_input(input: &str) -> BTreeMap<Coord, Sensor> {
        let (_, result) = separated_list1(line_ending, parse_line)(input).unwrap();

        result.into_iter().collect()
    }
}

fn get_footprint(map: &BTreeMap<Coord, Sensor>) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    if map.is_empty() {
        return ((0..=0), (0..=0));
    }

    let x_min = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.x - closest_beacon.distance)
        .min()
        .unwrap();
    let x_max = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.x + closest_beacon.distance)
        .max()
        .unwrap();
    let y_min = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.y - closest_beacon.distance)
        .min()
        .unwrap();
    let y_max = map
        .iter()
        .map(|(sensor, closest_beacon)| sensor.y + closest_beacon.distance)
        .max()
        .unwrap();

    ((x_min..=x_max), (y_min..=y_max))
}

fn get_row(
    row_number: i32,
    map: &BTreeMap<Coord, Sensor>,
    footprint: (RangeInclusive<i32>, RangeInclusive<i32>),
) -> Vec<MapState> {
    let (x_range, _) = footprint;
    let min_x = x_range.clone().min().unwrap();

    let mut result = vec![MapState::Unknown; x_range.count()];

    let filtered_data = map.iter().filter(|(pos_sensor, sensor)| {
        let distance: i32 = pos_sensor.y.abs_diff(row_number).try_into().unwrap();
        distance <= sensor.distance
    });

    for (position, sensor) in filtered_data {
        let distance: i32 = position.y.abs_diff(row_number).try_into().unwrap();
        let start: i32 = position.x - (sensor.distance - distance);
        let end: i32 = position.x + (sensor.distance - distance);
        for i in start..=end {
            result[(i - min_x) as usize] = MapState::Empty;
        }
        if position.y == row_number {
            result[(position.x - min_x) as usize] = MapState::Sensor;
        }
        if sensor.closest_beacon.y == row_number {
            result[(sensor.closest_beacon.x - min_x) as usize] = MapState::Beacon;
        }
    }

    result
}

fn get_first_empty_position(subgrid_size: i32, map: &BTreeMap<Coord, Sensor>) -> Option<Coord> {
    for row_number in 0..=subgrid_size {
        let ranges = map
            .iter()
            .map(|(position, sensor)| {
                let distance: i32 = position.y.abs_diff(row_number).try_into().unwrap();
                let start: i32 = position.x - (sensor.distance - distance);
                let end: i32 = position.x + (sensor.distance - distance);

                (start.max(0), end)
            })
            .sorted_by_key(|x| x.0);

        let mut covered_until = -1;
        for range in ranges {
            let min = range.0;
            let max = range.1;
            if max > subgrid_size {
                break;
            }

            if min > covered_until + 1 {
                return Some(Coord {
                    x: covered_until + 1,
                    y: row_number,
                });
            }

            covered_until = covered_until.max(max);
        }
    }
    None
}

pub fn puzzle_1(input: &str) -> String {
    let row_number = 2_000_000;

    let map = parser::parse_input(input);
    let footprint = get_footprint(&map);
    get_row(row_number, &map, footprint)
        .iter()
        .filter(|&&state| state == MapState::Empty)
        .count()
        .to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let max_coord = 4_000_000;

    let map = parser::parse_input(input);
    let first_empty_position = get_first_empty_position(max_coord, &map).unwrap();

    let value = first_empty_position.x as i64 * max_coord as i64 + first_empty_position.y as i64;
    value.to_string()
}

#[cfg(test)]
mod tests {

    fn get_input() -> String {
        let day = 15;
        let input_file_path = format!("../data/tests/test{:02}.txt", day);
        std::fs::read_to_string(input_file_path).unwrap()
    }

    use super::*;

    #[test]
    fn test_puzzle_1() {
        let row_number = 10;

        let input = get_input();
        let map = parser::parse_input(&input);
        let footprint = get_footprint(&map);
        let result = get_row(row_number, &map, footprint)
            .iter()
            .filter(|&&state| state == MapState::Empty)
            .count();

        assert_eq!(result, 26);
    }

    #[test]
    fn test_puzzle_2() {
        let max_coord = 20;

        let input = get_input();
        let map = parser::parse_input(&input);
        let first_empty_position = get_first_empty_position(max_coord, &map).unwrap();

        let value = first_empty_position.x as i64 * 4_000_000i64 + first_empty_position.y as i64;

        assert_eq!(value, 56000011);
    }
}
