use core::panic;
use std::{env, time::Instant};

use days::get_days;
use utils::Day;

mod days;
mod utils;

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
    let args: Vec<String> = env::args().collect();
    let year = args.get(1).and_then(|d| str::parse::<u8>(d).ok()).unwrap();
    let days = get_days(year);
    let day_number = args.get(2).and_then(|d| str::parse::<usize>(d).ok());
    let selected_days: Box<dyn Iterator<Item = &Day>> = match day_number {
        None => Box::new(days.iter()),
        Some(day_number) => Box::new(std::iter::once(days.get(day_number - 1).unwrap())),
    };
    let puzzle_number = args.get(3).and_then(|p| str::parse::<usize>(p).ok());

    for day in selected_days {
        let input_file = utils::get_input(utils::InputKind::Run, year, day.day).unwrap();

        if puzzle_number.is_none() || puzzle_number == Some(1) {
            execute(day, 1, &input_file);
        }
        if puzzle_number.is_none() || puzzle_number == Some(2) {
            execute(day, 2, &input_file);
        }
    }
}
