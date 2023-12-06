use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let mut grid = Grid { values: Vec::new() };
    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        let mut y = 0;
        let mut values: Vec<Value> = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut x = 0;
                for char in line.chars() {
                    values.push(Value {
                        coord: Coord {
                            x_coord: x,
                            y_coord: y,
                        },
                        value: char,
                    });

                    x += 1;
                }
                y += 1;
            }
        }
        grid.values = values;
    }

    part_one(&grid);

}

fn part_one(grid: &Grid) {
  let mut sum = 0;
  let mut symbol_coords: Vec<Coord> = Vec::new();
  for value in &grid.values {
    if value.value.is_ascii_punctuation() && value.value != '.' {
       symbol_coords.push(value.coord);
    }
  }
  for symbol_coord in &symbol_coords {
    //Check if the char above the coord is a number

    for value in &grid.values {
      if value.coord.x_coord == symbol_coord.x_coord && value.coord.y_coord == symbol_coord.y_coord - 1 {
        if value.value.is_ascii_digit() {
            //Check if there is numbers to the left and right of the coord and do this recursively to find the entire number
            let mut number = String::new();
            number.push(value.value);
            let mut x = value.coord.x_coord;
            let mut y = value.coord.y_coord;
            loop {
              let mut found = false;
              for value in &grid.values {
                if value.coord.x_coord == x - 1 && value.coord.y_coord == y {
                  if value.value.is_ascii_digit() {
                    number.push(value.value);
                    x -= 1;
                    found = true;
                    break;
                  }
                }
              }
              if !found {
                break;
              }
            }
            println!("Number: {}", number);
        
        }
      }
    }
  }
}

struct Grid {
    values: Vec<Value>,
}

struct Value {
   coord: Coord,
    value: char,
}

#[derive(Clone, Copy)] // Add the `Copy` trait to `Coord`
struct Coord {
    x_coord: i32,
    y_coord: i32,
}
