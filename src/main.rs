use lexer::Lexer;
use std::fs::File;
use std::io::Read;

mod lexer;

fn main() {
    let e_vec: Vec<usize> = file_to_chars("e_to_10k.txt")
        .iter()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect();

    let text = file_to_chars("check.txt");
    let lexer = Lexer::new(&text);

    /* TODO: Remake this logic vvvvv
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
    */
}

fn file_to_chars(path: &str) -> Vec<char> {
    let file = File::open(path).expect("Could not open file!");
    file.bytes()
        .into_iter()
        .map(|b| b.unwrap() as char)
        .collect::<Vec<char>>()
}
