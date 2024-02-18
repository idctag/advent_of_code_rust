use std::fs;

#[allow(unused)]
pub fn run() {
    let input = fs::read_to_string("data/day02.txt").unwrap();
    let all_games: Vec<_> = input.trim_end().lines().map(caliberate).collect();

    let possible_sum: u32 = all_games
        .iter()
        .filter(|(_, maxs)| is_possible(maxs))
        .map(|(id, _)| id)
        .sum();

    let powers: u32 = all_games.iter().map(|(_, maxs)| power(maxs)).sum();

    println!("part one: {}\npart two: {}", possible_sum, powers)
}
fn is_possible(maxs: &[u32; 3]) -> bool {
    maxs[0] <= 12 && maxs[1] <= 13 && maxs[2] <= 14
}
fn power(maxs: &[u32; 3]) -> u32 {
    maxs[0] * maxs[1] * maxs[2]
}

#[allow(unused)]
fn caliberate(input: &str) -> (u32, [u32; 3]) {
    let mut parts = input.split(":");
    let header = parts.next().expect("invalid line format (header)");
    let game_id: u32 = header["Game ".len()..]
        .parse()
        .expect("Invalid number (game_id)");
    let sets = parts.next().expect("Invalid line format (sets)");
    let mut maxs: [u32; 3] = [0, 0, 0];
    for n_color in sets.split(|c| c == ',' || c == ';') {
        let mut set_parts = n_color[1..].split_whitespace();
        let number: u32 = set_parts
            .next()
            .expect("Invalid line format (number)")
            .parse()
            .expect("Invalid number");
        let color = set_parts.next().expect("Invalid color");
        let index = match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => unreachable!(),
        };
        maxs[index] = number.max(maxs[index]);
        // println!("{:?}",)
        // println!("{:?}, {:?}", set_parts.next(), set_parts.next())
    }
    (game_id, maxs)
}
