use crate::error::Error;
pub trait DefaultParserTrait {
    fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool;

    fn consume_whitespace(&mut self);

    fn consume_char(&mut self) -> Result<char, Error>;

    fn next_char(&self) -> Result<char, Error>;

    fn starts_with(&self, s: &str) -> bool;

    fn eof(&self) -> bool;
}
