use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Open the file for reading
    let result = part_two();
    result
}

fn part_one() -> io::Result<()> {
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

fn part_two() -> io::Result<()> {
    let regex_set = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;

    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let mut first_digit = None;
                let mut last_digit = None;
                let mut first_index = i32::MAX;
                let mut last_index = i32::MIN;

                for (key, value) in regex_set.iter() {
                    if let Some(index) = line.find(key) {
                        let index_32 = index as i32;
                        if index_32 < first_index {
                            first_index = index_32;
                            first_digit = Some(value);
                        }
                    }

                    if let Some(index) = line.rfind(key) {
                        let index_32 = index as i32;
                        if index_32 > last_index {
                            last_index = index_32;
                            last_digit = Some(value);
                        }
                    }
                }

                println!("{:?} {:?} {:?}", first_digit, last_digit,line);

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
