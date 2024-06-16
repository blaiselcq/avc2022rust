use crate::utils::{get_day, Day};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;

pub fn get_days() -> Vec<Day> {
    vec![
        get_day!(1, day_01),
        get_day!(2, day_02),
        get_day!(3, day_03),
        get_day!(4, day_04),
        get_day!(5, day_05),
        get_day!(6, day_06),
        get_day!(7, day_07),
        get_day!(8, day_08),
        get_day!(9, day_09),
        get_day!(10, day_10),
        get_day!(11, day_11),
        get_day!(12, day_12),
        get_day!(13, day_13),
        get_day!(14, day_14),
        get_day!(15, day_15),
        get_day!(16, day_16),
        get_day!(17, day_17),
    ]
}
