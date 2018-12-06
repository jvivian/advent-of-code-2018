use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

/// Confident that your list of box IDs is complete, you're ready to find the boxes full of
/// prototype fabric. The boxes will have IDs which differ by exactly one character at the same
/// position in both strings.
///
/// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
/// However, the IDs fghij and fguij differ by exactly one character, the third (h and u).
/// Those must be the correct boxes.
///
/// What letters are common between the two correct box IDs?
/// (In the example above, this is found by removing the differing character from either ID,
/// producing fgij.)
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    find_crate(&input)?;

    Ok(())
}


fn find_crate(input: &str) -> Result<String> {

    // Define crate length as n-1 of one of the crates
    let first_line = input.lines().next().unwrap();
    let crate_len = first_line.trim().len() - 1;
    let a = String::new();

    // Iterate over lines
    for l1 in input.lines() {
        let l1_vec: Vec<char> = l1.chars().collect();
        for l2 in input.lines() {

            // If two strings are the same, continue
            if l1.trim() == l2.trim() { continue; }

            // Iterate over chars within lines and compare
            let mut new_word: Vec<char> = Vec::new();
            let l2_vec: Vec<char> = l2.chars().collect();
            for i in 0..=crate_len {
                if l1_vec[i] == l2_vec[i] { new_word.push(l1_vec[i]) }
            }

            if new_word.len() == crate_len {
                let new_word: String = new_word.iter().collect();
                writeln!(io::stdout(),
                         "Boxes found!\nWord 1: {}\nWord 2: {}\nCombined: {}",
                         l1, l2, new_word);
                return Ok(new_word);
            }
        }
    }
    Ok(a)
}
