use crate::utils::Day;
use core::panic;

pub mod y2021;
pub mod y2022;

pub fn get_days(year: u16) -> Vec<Day> {
    match year {
        2021 => y2021::get_days(),
        2022 => y2022::get_days(),
        _ => panic!("unhandled year {}", year),
    }
}
