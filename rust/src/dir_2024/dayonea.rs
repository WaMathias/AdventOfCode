use std::fs::File;
use std::io::{self, BufRead};

pub fn dayonea() -> io::Result<()> {
    // Pfad zur Eingabedatei
    let path = "/home/mathias/Documents/AdventOfCode/rust/input_dayone.txt"; // Ersetze dies durch den tatsächlichen Pfad
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Lies die beiden Listen ein
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if i == 0 {
            left_list = numbers;
        } else if i == 1 {
            right_list = numbers;
        }
    }

    // Sortiere beide Listen
    left_list.sort_unstable();
    right_list.sort_unstable();

    // Berechne die totale Distanz
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!(
        "Die Gesamtdistanz zwischen den beiden Listen ist: {}",
        total_distance
    );
    Ok(())
}
