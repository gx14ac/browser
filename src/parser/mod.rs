pub mod interface;

use error::Error;

#[derive(Debug)]
pub struct Parser {
    pub position: usize,
    pub source: String,
}

impl interface::DefaultParserTrait for Parser {
    fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while !self.eof() && f(self.next_char().unwrap()) {
            let consume_char = self.consume_char().unwrap();
            result.push(consume_char);
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_char(&mut self) -> Result<char, Error> {
        let mut char_indicies = self.source[self.position..].char_indices();
        let (_, cur_char) = char_indicies.next().unwrap();
        let (next_pos, _) = char_indicies.next().unwrap_or((1, ' '));
        self.position += next_pos;
        Ok(cur_char)
    }

    fn next_char(&self) -> Result<char, Error> {
        self.source[self.position..]
            .chars()
            .next()
            .ok_or(Error::ReadError)
    }

    fn starts_with(&self, s: &str) -> bool {
        self.source[self.position..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}
