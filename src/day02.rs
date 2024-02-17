use std::{collections::HashMap, fs};

pub fn day_two() {
    let input = fs::read_to_string("data/day02.txt").unwrap();

    let mut result2 = 0;
    let mut result1 = 0;
    for line in input.trim().lines() {
        result2 += caliberate_two(line);
        result1 += caliberate(line)
    }
    println!("one: {result1} \ntwo: {result2}")
}

fn caliberate_two(line: &str) -> u32 {
    let value: &str = line.split(":").collect::<Vec<_>>()[1];

    let games = seperate_games(value);

    let color_min = hash_games(games);

    let mut result = 1;
    for (_, &count) in &color_min {
        result *= count
    }
    result
}

fn caliberate(line: &str) -> u32 {
    let id_input: Vec<_> = line.split(":").collect();

    let game_id: u32 = id_input[0].split_whitespace().collect::<Vec<_>>()[1]
        .trim()
        .parse()
        .unwrap();

    let games = seperate_games(id_input[1]);

    for value in games {
        let is_valid: bool = check_valid(value);

        if !is_valid {
            return 0;
        }
    }

    game_id
}

fn hash_games(games: Vec<&str>) -> HashMap<String, u32> {
    let mut color_amount: HashMap<String, u32> = HashMap::new();

    for game in games {
        let entry: Vec<_> = game.split_whitespace().collect();
        let color = entry[1];
        let number: u32 = entry[0].parse().unwrap();

        if let Some(current_amount) = color_amount.get_mut(color) {
            if number > *current_amount {
                *current_amount = number
            }
        } else {
            color_amount.insert(color.to_string(), number);
        };
    }
    color_amount
}
fn seperate_games(games: &str) -> Vec<&str> {
    let colors: Vec<_> = games.trim().split(";").collect();

    let mut values: Vec<&str> = vec![];

    for color in colors {
        values.append(&mut color.trim().split(",").collect::<Vec<_>>())
    }

    for s in &mut values {
        *s = s.trim()
    }

    values
}

fn check_valid(value: &str) -> bool {
    let parts: Vec<&str> = value.split_whitespace().collect();

    let number: u32 = parts[0].parse().unwrap_or(0);
    let key: String = parts[1].to_string();

    if number > 14 && key == "blue" {
        return false;
    } else if number > 13 && key == "green" {
        return false;
    } else if number > 12 && key == "red" {
        return false;
    }
    true
}
