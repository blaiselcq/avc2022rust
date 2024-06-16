use crate::utils::Day;
use core::panic;

pub mod y2021;
pub mod y2022;

pub fn get_days(year: u8) -> Vec<Day> {
    match year {
        21 => y2021::get_days(),
        22 => y2022::get_days(),
        _ => panic!("unhandled year {}", year),
    }
}
