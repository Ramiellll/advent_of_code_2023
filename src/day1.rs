pub fn main() {
    let calibration_string = String::from(
        "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet",
    );
    let calibration_lines = calibration_string.lines();
    let mut sum: i32 = 0;
    for line in calibration_lines {
        let mut digits: Vec<i32> = Vec::new();
        for c in line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i32;
                digits.push(digit);
            }
        }
        let mut together = digits[0].to_string();
        let last_digit = digits[digits.len() - 1].to_string();
        together.push_str(&last_digit);
        sum += &together.parse::<i32>().unwrap();
    }
    println!("Sum of digits in line: {}", sum);
}