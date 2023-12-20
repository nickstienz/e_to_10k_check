pub struct Lexer<'a> {
    characters: &'a [char],
    position: usize,
    lexeme: String,
    word_len: usize,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a [char]) -> Self {
        Self {
            characters: text,
            position: 0,
            lexeme: String::new(),
            word_len: 0,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Option<(String, usize, usize)> {
        loop {
            let current = match self.characters.get(self.position) {
                Some(c) => c,
                None => return None,
            };
            self.position += 1;

            match current {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    self.word_len += 1;
                    self.lexeme.push(*current);
                }
                '\'' => {
                    self.lexeme.push(*current);
                }
                _ => {
                    if self.lexeme.len() > 0 {
                        self.index += 1;
                        let lexeme = self.lexeme.clone();
                        let word_len = self.word_len;
                        let words = self.index;
                        self.word_len = 0;
                        self.lexeme.clear();
                        return Some((lexeme, word_len, words));
                    }
                    self.word_len = 0;
                    self.lexeme.clear();
                }
            }
        }
    }
}
