use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Open the file for reading
    let mut sum = 0;
    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let first_digit = line.chars().find(|c| c.is_ascii_digit());
                let last_digit = line.chars().rev().find(|c| c.is_ascii_digit());
                if let (Some(first), Some(last)) = (first_digit, last_digit) {
                    let combined = format!("{}{}", first, last);
                    sum += combined.parse::<i32>().unwrap();
                    println!("{}", combined);
                }
            }
        }
    }
    println!("Sum: {}", sum);

    Ok(())
}
