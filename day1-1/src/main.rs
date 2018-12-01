/// After feeling like you've been falling for a few minutes, you look at the device's tiny screen.
/// "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain
/// destination lock." Below the message, the device shows a sequence of changes in frequency
/// (your puzzle input). A value like +6 means the current frequency increases by 6; a value like
/// -3 means the current frequency decreases by 3.
///
/// Starting with a frequency of zero, what is the resulting frequency after all of the changes in
/// frequency have been applied?

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    // Open file and create BufReader object which allows iteration over lines
    let f = File::open("input")
        .expect("Failed to open file");
    let f = BufReader::new(f);

    // Declare signed counter since we have both negative and positive values
    let mut counter: i32 = 0;

    // Iterate over lines and add/subtract based on first char in string
    for line in f.lines() {
        let line = line.expect("Failed to read line");

        // Extract and declare integer type here as compiler cannot infer type
        let val: i32 = line[1..].parse().unwrap();

        if line.chars().next().unwrap() == '+' {
            counter += val;
        } else {
            counter -= val;
        }
    }

    println!("Counter Value: {}", counter);
}
