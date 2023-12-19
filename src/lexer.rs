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

    pub fn next(&mut self) -> Option<&char> {
        let character = self.characters.get(self.position);
        self.position += 1;
        character
    }
}
