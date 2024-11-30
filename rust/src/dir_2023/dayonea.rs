use std::fs::File;
use std::io::{self, BufRead};

pub fn dayonea() -> io::Result<()> {
    let path = "/home/mathias/Documents/AdventOfCode/calibrations.txt";
    let input = File::open(&path)?;
    let reader = io::BufReader::new(input);

    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some(value) = extract_calibration_value_partone(&line) {
            total_sum += value;
        }
    }

    println!("Die Gesamtsumme der Kalibrierungswerte ist: {}", total_sum);

    Ok(())
}

fn extract_calibration_value_partone(line: &str) -> Option<i32> {
    let first_digit = line.chars().find(|c| c.is_digit(10))?;
    let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
    let combined_value = format!("{}{}", first_digit, last_digit)
        .parse::<i32>()
        .ok()?;
    Some(combined_value)
}
