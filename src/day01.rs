use std::{collections::HashMap, fs};

type Digits = HashMap<String, u32>;

pub fn run() {
    let digits: Digits = (0..=9).map(|i| (i.to_string(), i)).collect();
    let input = fs::read_to_string("data/day01.txt").unwrap();

    let result: u32 = input
        .lines()
        .map(|line| caliberate_value(line, &digits))
        .sum();

    println!("part 1: {}", &result);

    let digit_word: Digits = "one,two,three,four,five,six,seven,eight,nine"
        .split(",")
        .map(|s| s.to_string())
        .zip(1..=9)
        .chain(digits)
        .collect();

    let result: u32 = input
        .lines()
        .map(|line| caliberate_value(line, &digit_word))
        .sum();

    println!("part 2: {result}")
}

fn caliberate_value(line: &str, nums: &Digits) -> u32 {
    let first = nums
        .iter()
        .map(|(digit_string, &value)| (line.find(digit_string), value))
        .filter(|(index, _)| index.is_some())
        .min()
        .unwrap()
        .1;
    let second = nums
        .iter()
        .map(|(digit_string, &value)| (line.rfind(digit_string), value))
        .max()
        .unwrap()
        .1;
    first * 10 + second
}
