use std::time::Instant;

mod days;

struct Day {
    day: u8,
    puzzle_1: fn(&str) -> String,
    puzzle_2: fn(&str) -> String,
}

const DAYS: [Day; 14] = [
    Day {
        day: 1,
        puzzle_1: days::day_01::puzzle_1,
        puzzle_2: days::day_01::puzzle_2,
    },
    Day {
        day: 2,
        puzzle_1: days::day_02::puzzle_1,
        puzzle_2: days::day_02::puzzle_2,
    },
    Day {
        day: 3,
        puzzle_1: days::day_03::puzzle_1,
        puzzle_2: days::day_03::puzzle_2,
    },
    Day {
        day: 4,
        puzzle_1: days::day_04::puzzle_1,
        puzzle_2: days::day_04::puzzle_2,
    },
    Day {
        day: 5,
        puzzle_1: days::day_05::puzzle_1,
        puzzle_2: days::day_05::puzzle_2,
    },
    Day {
        day: 6,
        puzzle_1: days::day_06::puzzle_1,
        puzzle_2: days::day_06::puzzle_2,
    },
    Day {
        day: 7,
        puzzle_1: days::day_07::puzzle_1,
        puzzle_2: days::day_07::puzzle_2,
    },
    Day {
        day: 8,
        puzzle_1: days::day_08::puzzle_1,
        puzzle_2: days::day_08::puzzle_2,
    },
    Day {
        day: 9,
        puzzle_1: days::day_09::puzzle_1,
        puzzle_2: days::day_09::puzzle_2,
    },
    Day {
        day: 10,
        puzzle_1: days::day_10::puzzle_1,
        puzzle_2: days::day_10::puzzle_2,
    },
    Day {
        day: 11,
        puzzle_1: days::day_11::puzzle_1,
        puzzle_2: days::day_11::puzzle_2,
    },
    Day {
        day: 12,
        puzzle_1: days::day_12::puzzle_1,
        puzzle_2: days::day_12::puzzle_2,
    },
    Day {
        day: 13,
        puzzle_1: days::day_13::puzzle_1,
        puzzle_2: days::day_13::puzzle_2,
    },
    Day {
        day: 14,
        puzzle_1: days::day_14::puzzle_1,
        puzzle_2: days::day_14::puzzle_2,
    },
];

fn main() {
    for day in DAYS {
        let input_file_path = format!("../data/inputs/input{:02}.txt", day.day);
        let input_file = std::fs::read_to_string(input_file_path).unwrap();

        let start = Instant::now();
        let puzzle_1 = (day.puzzle_1)(&input_file);
        println!(
            "Day {} \t Time: {:.2e} s \t Puzzle 1: {}",
            day.day,
            start.elapsed().as_secs_f32(),
            puzzle_1
        );
        let puzzle_2 = (day.puzzle_2)(&input_file);
        println!(
            "Day {} \t Time: {:.2e} s \t Puzzle 2: {}",
            day.day,
            start.elapsed().as_secs_f32(),
            puzzle_2
        );
    }
}
