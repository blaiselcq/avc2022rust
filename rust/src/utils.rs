use std::path::PathBuf;

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
