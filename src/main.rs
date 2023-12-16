use std::fs::File;
use std::io::Read;

fn main() {
    // Get words in check.txt
    let mut check_file = File::open("check.txt").unwrap();
    let mut check_buffer = String::new();
    check_file.read_to_string(&mut check_buffer).unwrap();
    let words: Vec<&str> = check_buffer
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
        .collect();

    // Create Vec of 10,000 numbers from digits in e (Euler's number)
    let mut e_file = File::open("e_to_10k.txt").unwrap();
    let mut e_buffer = String::with_capacity(10_000);
    e_file.read_to_string(&mut e_buffer).unwrap();
    let e_vec: Vec<usize> = e_buffer
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .filter(|&d| d != 0)
        .collect();

    // Check if word length match digit in e
    for (pos, (word, &digit)) in words.iter().zip(e_vec.iter()).enumerate() {
        let len = word.len();
        if len != digit {
            println!(
                "Mismatch for Word at {}: {}, Length: {}, Digit: {}",
                pos + 1,
                word,
                len,
                digit
            );
        }
    }
}
