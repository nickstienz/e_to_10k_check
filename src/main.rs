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

    let args: Vec<String> = std::env::args().collect();
    let path = match args.get(1) {
        Some(p) => p,
        None => panic!("Argument needed: e_to_10k_check [path]"),
    };

    let text = file_to_chars(path);
    let mut lexer = Lexer::new(&text);

    loop {
        let (word, len, idx) = match lexer.next() {
            Some(w) => w,
            None => break,
        };

        if len != e_vec[idx - 1] {
            println!("Word {} ({}) {} => {}", idx, word, len, e_vec[idx - 1]);
        }
    }
}

fn file_to_chars(path: &str) -> Vec<char> {
    let file = File::open(path).expect("Could not open file!");
    file.bytes()
        .into_iter()
        .map(|b| b.unwrap() as char)
        .collect::<Vec<char>>()
}
