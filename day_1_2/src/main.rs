use std::{fs, str::Chars, cmp::min}; 

fn main() {
    let file_str = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    let nums_tex = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let text = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let nums = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for (j, line) in file_str.split("\n").enumerate() {
        if line != "" || j != 1000 {
            let mut low_index: (usize, &str) = (usize::MAX, "");
            for num in nums_tex {
                if let Some(index) = line.find(num) {
                    if index < low_index.0 {
                        low_index = (index, num);
                    }
                }
            }

            let mut first = low_index.1.to_string();
            for (i, num) in text.iter().enumerate() {
                first = first.replace(num, nums[i]).to_string();
            }

            let mut high_index = (usize::MAX, "");

            for num in nums_tex {
                if let Some(index) = line.chars().rev().collect::<String>().find(&num.chars().rev().collect::<String>()) {
                    if index < high_index.0 {
                        high_index = (index, num);
                    }
                }
            }

            let mut last = high_index.1.to_string();
            for (i, num) in text.iter().enumerate() {
                last = last.replace(num, nums[i]).to_string();
            }

            println!("{j}: {first}{last}");

            sum += format!("{first}{last}").parse::<i32>().unwrap();
        }
    }

    println!("{sum}");
}
