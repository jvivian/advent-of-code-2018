use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// You notice that the device repeats the same frequency change list over and over. To calibrate
/// the device, you need to find the first frequency it reaches twice.
///
/// Note that your device might need to repeat its list of frequency changes many times before a
/// duplicate frequency is found, and that duplicates might be found while in the middle of
/// processing the list.
///
/// We'll use a hashmap to store a counter for each frequency and when we reach 2 we stop and
/// display the duplicate frequency.
fn main() {
    let mut freq: i32 = 0;
    let mut iter_counter = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    loop {
        let f = File::open("input")
            .expect("Failed to open file");
        let f = BufReader::new(f);

        for line in f.lines() {
            let line = line.expect("Failed to read line");

            // Add value to frequency
            let val: i32 = line[1..].parse().unwrap();
            if line.chars().next().unwrap() == '+' {
                freq += val;
            } else {
                freq -= val;
            }

            // Store freq in hashmap
            let mut counter = map.entry(freq).or_insert(0);
            *counter += 1;
            if *counter == 2 {
                println!("Duplicate frequency found {}", counter);
                println!("Number of iterations to reach duplicate frequency: {}", iter_counter + 1);
                return;
            }
        }

        // Iter counter for funsies
        iter_counter += 1;
    }
}

