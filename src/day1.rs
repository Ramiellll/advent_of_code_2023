use regex::{Regex, Captures};
use std::{fs, ops::AddAssign};

pub fn main() {
    let calibration_string = fs::read_to_string("./input/day1.txt").expect("Could not read file");
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    let calibration_lines = calibration_string.lines();
    let mut sum: i32 = 0;
    for line in calibration_lines {
        let new_line = re.replace_all(line, |caps: &Captures| {
            match &caps[0] {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => panic!("Unknown digit"),
            }
        });
        println!("new_line: {}", new_line);
        let mut digits: Vec<i32> = Vec::new();
        for c in new_line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i32;
                digits.push(digit);
            }
        }
        let mut together = digits[0].to_string();
        let last_digit = digits[digits.len() - 1].to_string();
        together.push_str(&last_digit);
        println!("sum: {}", sum);
        sum += together.parse::<i32>().unwrap();
        // sum += &together.parse::<i32>();
    }
    println!("Sum of digits in line: {}", sum);
}