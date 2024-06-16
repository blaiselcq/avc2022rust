use std::path::PathBuf;

#[allow(dead_code)]
pub enum InputKind {
    Test,
    Run,
}

/// year must be the avc edition (minus 2000)
pub fn get_input(kind: InputKind, year: u8, day: u8) -> Result<String, std::io::Error> {
    let dir = match kind {
        InputKind::Test => PathBuf::from("../data/tests"),
        InputKind::Run => PathBuf::from("../data/inputs"),
    };
    let input_file_path = dir.join(format!("20{}/{:02}.txt", year, day));

    std::fs::read_to_string(input_file_path)
}

pub struct Day {
    pub day: u8,
    pub puzzle_1: fn(&str) -> String,
    pub puzzle_2: fn(&str) -> String,
}

macro_rules! get_day {
    ($n: tt, $mod:tt) => {
        Day {
            day: $n,
            puzzle_1: $mod::puzzle_1,
            puzzle_2: $mod::puzzle_2,
        }
    };
}
pub(crate) use get_day;
