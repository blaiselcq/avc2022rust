use core::panic;
use std::{env, time::Instant};

mod days;
mod utils;

struct Day {
    day: u8,
    puzzle_1: fn(&str) -> String,
    puzzle_2: fn(&str) -> String,
}

macro_rules! get_day {
    ($n: tt, $mod:tt) => {
        Day {
            day: $n,
            puzzle_1: days::$mod::puzzle_1,
            puzzle_2: days::$mod::puzzle_2,
        }
    };
}

fn execute(day: &Day, puzzle_number: u8, input: &String) {
    let start = Instant::now();
    let puzzle = match puzzle_number {
        1 => day.puzzle_1,
        2 => day.puzzle_2,
        _ => panic!(),
    };

    let result = puzzle(&input);

    println!(
        "Day {:02} \t Time: {:.2e} s \t Puzzle {}: {}",
        day.day,
        start.elapsed().as_secs_f32(),
        puzzle_number,
        result
    );
}

fn main() {
    let days = vec![
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
    ];

    let args: Vec<String> = env::args().collect();
    let day_number = args.get(1).and_then(|d| str::parse::<usize>(d).ok());
    let selected_days: Box<dyn Iterator<Item = &Day>> = match day_number {
        None => Box::new(days.iter()),
        Some(day_number) => Box::new(std::iter::once(days.get(day_number - 1).unwrap())),
    };
    let puzzle_number = args.get(2).and_then(|p| str::parse::<usize>(p).ok());

    for day in selected_days {
        let input_file = utils::get_input(utils::InputKind::Run, 22, day.day).unwrap();

        if puzzle_number.is_none() || puzzle_number == Some(1) {
            execute(day, 1, &input_file);
        }
        if puzzle_number.is_none() || puzzle_number == Some(2) {
            execute(day, 2, &input_file);
        }
    }
}
