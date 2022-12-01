use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut file = File::open("resources/day_1.txt").expect("Could not open input");
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut current_highest = 0;
    let mut current_calories = 0;

    loop {
        buffer.clear();

        let bytes_read = reader.read_line(&mut buffer).expect("Failed to read");

        if bytes_read == 0 {
            break;
        }

        // Process
        if buffer == "\r\n" {
            // Flush the current elf and compare to last known max
            if current_calories > current_highest {
                current_highest = current_calories;
            }

            current_calories = 0;
        } else {
            // Build elf
            current_calories += &buffer[..buffer.len() - 2].parse::<i32>().expect("Not a number");
        }
    }

    println!("Highest = {}", current_highest);
}
