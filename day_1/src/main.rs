use std::{fs, str::Chars}; 

fn main() {
    let file_str = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in file_str.split("\n") {
        if line != "" {
            let iter = line.chars().filter(|char| char.is_ascii_digit());
            println!("{iter:?}");
            let first = line.chars().filter(|char| char.is_ascii_digit()).nth(0).unwrap();
            let last = line.chars().filter(|char| char.is_ascii_digit()).last().unwrap();

            sum += format!("{first}{last}").parse::<i32>().unwrap();
        }
    }

    println!("{sum}");
}
