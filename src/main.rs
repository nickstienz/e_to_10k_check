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

    let mut lexeme = String::new();
    let mut word_len = 0;
    let mut words: Vec<(String, usize)> = Vec::new();
    loop {
        let current = match lexer.next() {
            Some(c) => c,
            None => break,
        };

        match current {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                word_len += 1;
                lexeme.push(*current);
            }
            '\'' => {
                lexeme.push(*current);
            }
            _ => {
                if lexeme.len() > 0 {
                    words.push((lexeme.clone(), word_len));
                }
                word_len = 0;
                lexeme.clear();
            }
        }
    }

    let mut found = false;
    words.iter().enumerate().for_each(|(idx, (word, len))| {
        if *len != e_vec[idx] {
            found = true;
            println!("Found ({}) {} => {}", word, len, e_vec[idx])
        }
    });

    if !found {
        println!("No issues found!");
    }
}

fn file_to_chars(path: &str) -> Vec<char> {
    let file = File::open(path).expect("Could not open file!");
    file.bytes()
        .into_iter()
        .map(|b| b.unwrap() as char)
        .collect::<Vec<char>>()
}
