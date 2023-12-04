use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let cards = part_one();
    part_two(cards);
}

fn part_one() -> Vec<Card> {
    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        let mut cards: Vec<Card> = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                // Card 1: 58 6 71 93 96 38 25 29 17 8 | 79 33 93 58 53 96 71 8 67 90 17 6 46 85 64 25 73 32 18 52 77 16 63 2 38
                // Card number is what is after Card in the line
                let card_number = line.split(":").collect::<Vec<&str>>()[0]
                    .split(" ")
                    .collect::<Vec<&str>>()[1]
                    .parse::<u8>()
                    .unwrap();
                // Card numbers are what is after the first | in the line they need to be u8
                let card_numbers = line.split("|").collect::<Vec<&str>>()[1]
                    .split(" ")
                    .collect::<Vec<&str>>();
                let mut card_numbers_u8: Vec<u8> = Vec::new();

                for number in card_numbers.iter() {
                    if number == &"" {
                        continue;
                    }
                    card_numbers_u8.push(number.parse::<u8>().unwrap());
                }

                // Winning numbers are what is before the first | in the line and after the :
                let winning_numbers = line.split(":").collect::<Vec<&str>>()[1]
                    .split("|")
                    .collect::<Vec<&str>>()[0]
                    .split(" ")
                    .collect::<Vec<&str>>();
                let mut winning_numbers_u8: Vec<u8> = Vec::new();
                for number in winning_numbers.iter() {
                    if number == &"" {
                        continue;
                    }
                    winning_numbers_u8.push(number.parse::<u8>().unwrap());
                }

                let mut card = Card {
                    card_number: card_number,
                    winning_numbers: winning_numbers_u8,
                    card_numbers: card_numbers_u8,
                    points: 0,
                    actual_winning_cards: 0,
                };
                card.calculate_points();
                cards.push(card);
            }
        }
        let mut total_points = 0;
        for card in &cards {
            println!("Card number: {} Points: {}", card.card_number, card.points);
            total_points = total_points + card.points;
        }
        println!("Total points: {}", total_points);
        cards // Return the ownership of `cards`
    } else {
        panic!("Could not open file");
    }
}

fn part_two(cards: Vec<Card>) {}

struct Card {
    card_number: u8,
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
    actual_winning_cards: u8,
    points: i32,
}

impl Card {
    fn calculate_points(&mut self) {
        let mut points = 0;
        let mut winning_cards: u8 = 0;
        for number in self.card_numbers.iter() {
            if self.winning_numbers.contains(number) {
                winning_cards = winning_cards + 1;
                if (points == 0) {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }
        self.actual_winning_cards = winning_cards;
        self.points = points;
    }
}
