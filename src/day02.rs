use std::collections::HashMap;

#[allow(unused)]
type Game = HashMap<u32, HashMap<String, u32>>;

#[allow(unused)]
pub fn day_two() {
    let mut input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let mut colon = input.find(":").unwrap();
    let mut space = input.find(" ").unwrap() + 1;

    let mut values = &input[colon + 1..];
    let mut id: &u32 = &input[space..colon].parse().unwrap();
    // for i in vec {
    //     println!("{:?}", i)
    // }
    println!("{:#?}", values)
}
