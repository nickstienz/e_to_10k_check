pub struct Lexer<'a> {
    characters: &'a [char],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a [char]) -> Self {
        Self {
            characters: text,
            position: 0,
        }
    }
}
