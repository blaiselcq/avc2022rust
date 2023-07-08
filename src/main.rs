mod days;

struct Day {
    day: u8,
    puzzle_1: fn(&str) -> String,
    puzzle_2: fn(&str) -> String,
}

const DAYS: [Day; 6] = [
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
];

fn main() {
    for day in DAYS {
        let input_file_path = format!("./data/inputs/input{:02}.txt", day.day);
        let input_file = std::fs::read_to_string(input_file_path).unwrap();
        let puzzle_1 = (day.puzzle_1)(&input_file);
        let puzzle_2 = (day.puzzle_2)(&input_file);
        println!("Day {} - Puzzle 1: {}", day.day, puzzle_1);
        println!("Day {} - Puzzle 2: {}", day.day, puzzle_2);
    }
}
