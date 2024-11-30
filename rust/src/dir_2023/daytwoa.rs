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
    let parts: Vec<&str> = line.split(":").collect();

    // Zwischenspeichere das Ergebnis von `replace` in einer eigenen Variablen
    let binding = parts[0].replace("Game", "");
    let game_id_str = binding.trim(); // Jetzt trim() auf die zwischengespeicherte Variable anwenden

    let game_id: i32 = match game_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Ungültige Spiel-ID: {}", game_id_str);
            return None; // Rückgabe None, wenn die Spiel-ID nicht korrekt ist
        }
    };

    // Den Rest der Zeile behandeln (also die Würfeldaten)
    let game_data = parts.get(1).unwrap_or(&"").trim(); // Der Rest nach dem ":" symbolisiert die Würfeldaten
    let mut possible = true;

    for part in game_data.split(";") {
        let cubes: Vec<&str> = part.split(",").collect();
        for cube in cubes {
            let cube_info: Vec<&str> = cube.split_whitespace().collect();

            // Versuche, die Anzahl der Würfel zu parsen und handhabe Fehler
            let count: Result<u32, _> = cube_info[0].parse();
            let count = match count {
                Ok(c) => c, // Falls erfolgreich, verwende die Zahl
                Err(_) => {
                    eprintln!("Ungültiger Wert für die Würfelanzahl: {}", cube_info[0]);
                    possible = false;
                    continue; // Gehe zur nächsten Iteration
                }
            };

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
                _ => {} // Falls es noch andere Farben gibt
            }
        }
    }

    // Rückgabe der Spiel-ID, wenn das Spiel möglich ist
    if possible {
        Some(game_id) // Gib die Spiel-ID zurück, wenn das Spiel möglich ist
    } else {
        None
    }
}
