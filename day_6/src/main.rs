use std::io::{self, BufRead};

fn main() {
    let mut races: Vec<Race> = Vec::new();
    if let Ok(file) = std::fs::File::open("input.txt") {
        let reader = io::BufReader::new(file);
        let mut times: Vec<i64> = Vec::new();
        let mut distances: Vec<i64> = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
               let values = line.split(":").collect::<Vec<&str>>()[1];
                if line.contains("Time")
                {
                  
                    times = values.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                }
                else {
    
                    distances = values.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                }
            }
        }
        for (i, time) in times.iter().enumerate()  {
            races.push(Race {time: *time, distance: distances[i]});
        }
       
    } else {
        panic!("Could not open file");
    }
   
   part_two(races);
}

fn part_one(races : Vec<Race>) {
    let mut ways: Vec<i32> = Vec::new();
    for race in races {
        let mut way = 0;
        let mut time = 0;
        let mut speed = 0;
        while time < race.time {

            
            if ((race.time - time) * speed) > race.distance {
              
                way +=1;
            }
            time += 1;
            speed += 1;
        }
        ways.push(way);

    }
    //Multiply the values in the vector

    println!("Part one: {:?}", ways);
}

fn part_two(mut races : Vec<Race>)
{
    //Concat all the race times
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();
    for race in &races {
        times.push(race.time);
        distances.push(race.distance);
    }
    let concatenated_times: String = times.into_iter().map(|x| x.to_string()).collect();
    let concatenated_distances: String = distances.into_iter().map(|x| x.to_string()).collect();
    // Parse the concatenated string as an integer
    let result_time: i64 = concatenated_times.parse().unwrap();
    let result_distance: i64 = concatenated_distances.parse().unwrap();
    races.push(Race{ time: result_time, distance: result_distance});
    part_one(races.clone());
}

#[derive(Debug,Clone,Copy)] 
struct Race {
    time: i64,
    distance :i64,
}