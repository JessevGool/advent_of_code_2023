use std::fs::File;
use std::io::{self, BufRead};

// Game 100: 5 blue, 5 green; 7 blue, 15 green; 4 red, 7 green, 12 blue; 7 green, 1 blue; 5 blue, 9 green, 1 red

fn main() {
    let mut game_array: Vec<Game> = Vec::new();

    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts = line.split(":").collect::<Vec<&str>>();
                let mut game = Game {
                    number: 0,
                    greendice: 0,
                    reddice: 0,
                    bluedice: 0,
                };
                let game_number = parts[0].split(" ").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap();
                game.number = game_number;
                let different_sets = parts[1].split(";").collect::<Vec<&str>>();
                for set in different_sets {
                    let dice = set.split(",").collect::<Vec<&str>>();
                    for die in dice {
                        let die_parts = die.trim().split(" ").collect::<Vec<&str>>();
                        let die_color = die_parts[1];
                        let die_number = die_parts[0].parse::<i32>().unwrap();
                        if die_color == "green" {
                            if die_number > game.greendice {
                                game.greendice = die_number;
                            }
                        } else if die_color == "red" {
                            if die_number > game.reddice {
                                game.reddice = die_number;
                            }
                        } else if die_color == "blue" {
                            if die_number > game.bluedice {
                                game.bluedice = die_number;
                            }
                        }
                    }
                }
                game_array.push(game);
            }
        }
    }
    let mut sum = 0;
    for game in &game_array {
        if(game.reddice <= 12 && game.bluedice <= 14 && game.greendice <= 13) {
            sum += game.number;
        }
        
    }
    println!("Sum: {}", sum);
    let mut sum_part_two = 0;
    for game in game_array {
        sum_part_two += game.greendice * game.reddice * game.bluedice;
        
    }
    println!("Sum 2: {}", sum_part_two);
}

struct Game {
    number: i32,
    greendice: i32,
    reddice: i32,
    bluedice: i32,
}
