use std::fs::File;
use std::io::{self, BufRead};

pub fn dayonea() -> io::Result<()> {
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

    if left_list.is_empty() || right_list.is_empty() {
        eprintln!("Eine oder beide Listen sind leer. Überprüfen Sie die Eingabedatei.");
        return Ok(());
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    if left_list.len() < right_list.len() {
        left_list.resize(right_list.len(), 0);
    } else if right_list.len() < left_list.len() {
        right_list.resize(left_list.len(), 0);
    }

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
