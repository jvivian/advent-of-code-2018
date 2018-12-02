use std::io::{self, Result, Read, Write};
type Result<T> = Result<T, Box<std::error::Error>>;

/// After feeling like you've been falling for a few minutes, you look at the device's tiny screen.
/// "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain
/// destination lock." Below the message, the device shows a sequence of changes in frequency
/// (your puzzle input). A value like +6 means the current frequency increases by 6; a value like
/// -3 means the current frequency decreases by 3.
///
/// Starting with a frequency of zero, what is the resulting frequency after all of the changes in
/// frequency have been applied?



fn main() -> Result<()> {

    let mut freq = 0;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        let value: i32 = line.parse()?;
        freq += value;
    }

    writeln!(io::stdout(), "Frequency is: {}", freq)?;
    Ok(freq)
}



