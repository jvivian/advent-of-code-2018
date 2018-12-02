use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

/// You notice that the device repeats the same frequency change list over and over. To calibrate
/// the device, you need to find the first frequency it reaches twice.
///
/// Note that your device might need to repeat its list of frequency changes many times before a
/// duplicate frequency is found, and that duplicates might be found while in the middle of
/// processing the list.
///
/// We'll use a hashmap to store a counter for each frequency and when we reach 2 we stop and
/// display the duplicate frequency.
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    find_duplicate_freq(&input)?;
    Ok(())
}


fn find_duplicate_freq(input: &str) -> Result<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    let mut freq = 0;
    let mut counter = 1;

    loop {
        for line in input.lines() {
            let val: i32 = line.parse()?;
            freq += val;

            if set.contains(&freq) {
                writeln!(io::stdout(), "Number of input iterations: {}", counter)?;
                writeln!(io::stdout(), "Duplicated Frequency: {}", freq)?;
                return Ok(freq)
            }

            set.insert(freq);
        }

        counter += 1;
    }
}
