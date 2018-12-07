use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

/// The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.
///
/// Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims
/// have an ID and consist of a single rectangle with edges parallel to the edges of the fabric.

/// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the
/// left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall.
///
/// If the Elves all proceed with their own plans, none of them will have enough fabric.
/// How many square inches of fabric are within two or more claims?
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = calculate_duplicate_squares(&input)?;

    identify_non_overlap(&input, &map)?;

    Ok(())
}


/// Instead of storing a vector of vectors to represent an array, we can store a HashMap of (x, y)
/// tuple coordinates as keys. After placing all sheets, we count the number of values that are >=2.
fn calculate_duplicate_squares(input: &str) -> Result<HashMap<(u32, u32), u32>> {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    // parse line. line[2] is start coordinates and line[3] are cloth dimensions
    for line in input.lines() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();

        // Parse start coordinates
        let start: Vec<&str> = line[2].split(',').collect();
        let left: u32 = start[0].parse()?;
        let len = start[1].len();
        let top: u32 = start[1][..len - 1].parse()?;

        // Parse dimensions of cloth
        let dims: Vec<&str> = line[3].split('x').collect();
        let right: u32 = dims[0].parse()?;
        let down: u32 = dims[1].parse()?;

        // Store tuples in map
        for i in left..left + right {
            for j in top..top + down {
                let counter = map.entry((i, j)).or_insert(0);
                *counter += 1;
            }
        }
    }

    // Count number of squares that have greater than 1 value
    {
        let test: Vec<&u32> = map.values().filter(|&x| *x >= 2).collect();
        let overlap: u32 = test.len() as u32;
        writeln!(io::stdout(), "Number of overlapping squares: {}", overlap);
    }
    Ok(map)
}


/// Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch
/// of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be
/// able to make Santa's suit after all!
///
/// What is the ID of the only claim that doesn't overlap?
///
/// Solution, albeit non-optimal: Iterate over every sample and check if all values in the map
/// are equal to 1.
fn identify_non_overlap<'a>(input: &'a str, map: &'a HashMap<(u32, u32), u32>) -> Result<&'a str> {
    'outer: for line in input.lines() {
        let mut continue_flag = false;
        let line: Vec<&str> = line.trim().split_whitespace().collect();

        let sample_id = line[0];

        // Parse start coordinates
        let start: Vec<&str> = line[2].split(',').collect();
        let left: u32 = start[0].parse()?;
        let len = start[1].len();
        let top: u32 = start[1][..len - 1].parse()?;

        // Parse dimensions of cloth
        let dims: Vec<&str> = line[3].split('x').collect();
        let right: u32 = dims[0].parse()?;
        let down: u32 = dims[1].parse()?;

        // Check tuples for values greater than 1
        for i in left..left + right {
            for j in top..top + down {
                let val: u32 = map[&(i, j)];
                if val > 1 {
                    continue 'outer;
                }
            }
        }

        // If we make it here, then we've found our answer!
        writeln!(io::stdout(), "The correct ID is: {}", sample_id);
        return Ok(sample_id)
    }

    Ok("I don't know how to return Err")
}