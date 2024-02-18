use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/temp.txt").unwrap();

    let array: Vec<&str> = input.lines().collect();
    println!("{:#?}", array)
}
