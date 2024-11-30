use std::fs::File;
use std::io::{self, BufRead};

pub fn daytwoa() -> io::Result<()> {
    let path = "/home/mathias/Documents/AdventOfCode/input_day_two.txt"; // Ersetze dies durch den tatsächlichen Pfad
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut total_sum = 0;

    // Maximale Anzahl der verfügbaren Würfel:
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    for line in reader.lines() {
        let line = line?;
        if let Some(game_id) = check_game_possible(&line, max_red, max_green, max_blue) {
            total_sum += game_id;
        }
    }

    println!("Summe der IDs der gültigen Spiele: {}", total_sum);
    Ok(())
}

fn check_game_possible(line: &str, max_red: u32, max_green: u32, max_blue: u32) -> Option<i32> {
    let parts: Vec<&str> = line.split(";").collect();
    let mut possible = true;

    for part in parts {
        let cubes: Vec<&str> = part.split(",").collect();
        for cube in cubes {
            let cube_info: Vec<&str> = cube.split_whitespace().collect();
            let count: u32 = cube_info[0].parse().unwrap();
            let color = cube_info[1];

            match color {
                "red" => {
                    if count > max_red {
                        possible = false;
                    }
                }
                "green" => {
                    if count > max_green {
                        possible = false;
                    }
                }
                "blue" => {
                    if count > max_blue {
                        possible = false;
                    }
                }
                _ => {}
            }
        }
    }

    if possible {
        Some(line.parse().unwrap()) // Hier könnte die Game-ID extrahiert werden
    } else {
        None
    }
}
