use std::fs;

struct Game {
    id: u32,
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let file_str = fs::read_to_string("input.txt").unwrap();
    let mut game_vec: Vec<Game> = Vec::new();
    for line in file_str.split("\n") {
        if line != "" {
            let mut iter = line.split_whitespace();
            let id = iter.nth(1).unwrap().replace(":", "");
            println!("{id}");
            for color in iter.collect::<Vec<&str>>().chunks(2) {
                if let [num, color] = color {
                    let color = color.replace(",", "");
                    println!("{num} {color}");
                };     
            }
        }
    }
}
