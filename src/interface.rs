use dom;
use error;

pub trait HtmlParserTrait {
    fn parse_nodes(&mut self) -> Vec<dom::Node>;

    fn parse_node(&mut self) -> dom::Node;

    fn parse_text(&mut self) -> dom::Node;

    fn parse_element(&mut self) -> dom::Node;

    fn parse_attr(&mut self) -> Result<(String, String), error::Error>;

    fn parse_attr_value(&mut self) -> String;

    fn parse_attributes(&mut self) -> dom::AttrMap;

    fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool;

    fn parse_tag_name(&mut self) -> String;

    fn consume_whitespace(&mut self);

    // 現在の位置の文字列を取得し、位置を進める
    fn consume_char(&mut self) -> Result<char, error::Error>;

    fn to_next_position(&mut self) -> Result<(), error::Error>;

    fn get_current_char(&self) -> Result<(usize, char), error::Error>;

    fn next_char(&self) -> Result<char, error::Error>;

    fn starts_with(&self, s: &str) -> bool;

    fn eof(&self) -> bool;
}
