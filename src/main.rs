use std::fs::File;
use std::io::Read;

mod lexer;

fn main() {
    // Get words in the input file
    let text = file_to_chars("check.txt");
    let words: Vec<&str> = parse(String::from("test"));

    // Create Vec of 10,000 numbers from digits in e (Euler's number)
    let e_chars = file_to_chars("e_to_10k.txt");
    let e_vec: Vec<usize> = e_chars
        .iter()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
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

fn parse(t: String) -> Vec<&'static str> {
    vec!["Test"]
}

fn file_to_chars(path: &str) -> Vec<char> {
    let file = File::open(path).expect("Could not open file!");
    file.bytes()
        .into_iter()
        .map(|b| b.unwrap() as char)
        .collect::<Vec<char>>()
}
