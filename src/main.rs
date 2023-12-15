use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read the contents of check.txt
    let check_file = File::open("check.txt").expect("Unable to open check.txt");
    let check_reader = BufReader::new(check_file);

    // Read the first 10k digits of e from e_to_10k.txt
    let e_file = File::open("e_to_10k.txt").expect("Unable to open e_to_10k.txt");
    let e_reader = BufReader::new(e_file);

    // Check if the length of each word matches the corresponding digit in e
    for (line_num, (check_line, e_line)) in check_reader.lines().zip(e_reader.lines()).enumerate() {
        let check_content = check_line.expect("Error reading line from check.txt");
        let e_content = e_line.expect("Error reading line from e_to_10k.txt");

        let words: Vec<&str> = check_content
            .split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .collect();
        let e_digits: Vec<char> = e_content.chars().take(10000).collect();

        for (word, &digit) in words.iter().zip(e_digits.iter()) {
            let word_length = word.len();

            if word_length != digit.to_digit(10).expect("Invalid digit in e") as usize {
                println!(
                    "Line {}: Mismatch for Word: {}, Length: {}, Digit: {}",
                    line_num + 1,
                    word,
                    word_length,
                    digit
                );
            }
        }
    }
}
