use std::{fs, usize};

pub fn day_one() {
    let input = fs::read_to_string("data/day1.txt").unwrap();
    let v = count_sum(&input as &str);
    println!("{:?}", v)
}

fn count_sum(input: &str) -> usize {
    let lines: Vec<&str> = input.rsplit("\n").collect();
    let mut line_nums: Vec<usize> = vec![];
    for line in lines {
        let mut buffer = String::new();
        for c in line.chars() {
            match c.to_string().parse::<usize>() {
                Ok(_) => buffer.push(c),
                Err(_) => (),
            }
        }
        if buffer.len() > 2 {
            buffer = format!(
                "{}{}",
                buffer.chars().next().unwrap(),
                buffer.chars().last().unwrap()
            )
        } else if buffer.len() == 1 {
            buffer = format!("{}{}", buffer, buffer)
        }
        match buffer.to_string().parse::<usize>() {
            Ok(n) => line_nums.push(n),
            Err(_) => (),
        }
    }
    line_nums.iter().sum()
}
