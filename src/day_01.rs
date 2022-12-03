use std::fs::File;
use std::io::{BufRead, BufReader};

#[inline(always)]
fn is_newline(line: &str) -> bool {
    line.trim().is_empty()
}

#[derive(Debug)]
struct HighestStore {
    values: [i32; 3]
}

impl HighestStore {
    pub fn new() -> Self {
        Self {
            values: [0; 3]
        }
    }

    pub fn try_add(&mut self, value: i32) {
        for i in 0..3 {
            if value > self.values[i] {
                // Shift other values
                for j in i + 1..3 {
                    self.values[j] = self.values[i];
                }
                self.values[i] = value;
                return;
            }
        }
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}

pub fn run() {
    let file = File::open("resources/day_01.txt").expect("Could not open input");
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut store = HighestStore::new();

    // let mut current_highest = 0;
    let mut current_calories = 0;

    loop {
        buffer.clear();

        let bytes_read = reader.read_line(&mut buffer).expect("Failed to read");

        if bytes_read == 0 {
            break;
        }

        // Process
        if is_newline(&buffer) {
            // Flush the current elf and compare to last known max
            store.try_add(current_calories);

            current_calories = 0;
        } else {
            // Build elf
            current_calories += &buffer[..buffer.len() - 2].parse::<i32>().expect("Not a number");
        }
    }

    println!("Store = {:?} -> sum = {:?}", &store, &store.sum());
}
