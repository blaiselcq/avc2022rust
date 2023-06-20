fn load_input_data(input: &str) -> Vec<Vec<u32>> {
    let elf_day_calories = input.split("\n\n");
    elf_day_calories
        .map(|subdata| -> Vec<u32> {
            subdata
                .lines()
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_total_calories_by_elf(calories: Vec<Vec<u32>>) -> Vec<u32> {
    calories
        .iter()
        .map(|day_meal| day_meal.iter().sum())
        .collect()
}

pub fn puzzle_1(input: &str) -> u32 {
    let calories = load_input_data(input);
    let total_calories = get_total_calories_by_elf(calories);

    match total_calories.iter().max() {
        Some(max) => *max,
        None => 0,
    }
}

pub fn puzzle_2(input: &str) -> u32 {
    let calories = load_input_data(input);
    let mut total_calories = get_total_calories_by_elf(calories);
    assert!(total_calories.len() >= 3);

    total_calories.sort();

    let top_three = &total_calories[total_calories.len() - 3..];
    top_three.iter().sum()
}
