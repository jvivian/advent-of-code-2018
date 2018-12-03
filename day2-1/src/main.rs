use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

/// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number
/// that have an ID containing exactly two of any letter and then separately counting those with
/// exactly three of any letter. You can multiply those two counts together to get a rudimentary
/// checksum and compare it to what your device predicts.
/// Multiplying these together produces a checksum.
/// What is the checksum for your list of box IDs?
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    checksum(&input)?;

    Ok(())
}


fn checksum(input: &str) -> Result<i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    // Convert to vector of chars
    for line in input.lines() {
        let char_vec = line.chars();
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for c in char_vec {
            let count = char_map.entry(c).or_insert(0);
            *count += 1
        }

        // Now check if there are any occurrences of where the value is exactly 2 or exactly 3
        // but only count them at most once each
        if char_map.values().any(|&x| x == 2) == true {
            let count = count_map.entry(2).or_insert(0);
            *count += 1;
        }
        if char_map.values().any(|&x| x == 3) == true {
            let count = count_map.entry(3).or_insert(0);
            *count += 1;
        }
    }

    let product = count_map[&2] * count_map[&3];
    writeln!(io::stdout(), "There were {} '3's and {} 2's' in the input", count_map[&2], count_map[&3])?;
    writeln!(io::stdout(), "Checksum of input is: {}", product)?;

    Ok(product)
}
