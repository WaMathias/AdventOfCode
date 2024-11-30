use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn b_part() -> Result<i32, std::io::Error> {
    let filename = "/home/mathias/Documents/AdventOfCode/calibrations.txt"; // Passe den Dateinamen an, wenn nötig
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        let mut digits = Vec::new();

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
            } else {
                for (word, digit) in [
                    ("one", 1),
                    ("two", 2),
                    ("three", 3),
                    ("four", 4),
                    ("five", 5),
                    ("six", 6),
                    ("seven", 7),
                    ("eight", 8),
                    ("nine", 9),
                ] {
                    if line[i..].starts_with(word) {
                        digits.push(digit);
                        break;
                    }
                }
            }
        }

        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        let calibration_value = first_digit * 10 + last_digit;
        sum += calibration_value;
    }

    println!("Sum: {}", sum);
    Ok(sum as i32) // Entferne das abschließende Semikolon hier
}
