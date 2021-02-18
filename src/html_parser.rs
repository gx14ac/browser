use dom;
use error;
use interface::{self, HtmlParserTrait};
use std::collections::HashMap;

#[derive(Debug)]
struct HtmlParser {
    position: usize,
    source: String,
}

pub fn NewHtmlParser(source: String) -> impl interface::HtmlParserTrait {
    return HtmlParser {
        position: 0,
        source: source,
    };
}

impl interface::HtmlParserTrait for HtmlParser {
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes: Vec<dom::Node> = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
            println!("{:?}", nodes);
        }
        nodes
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char().unwrap() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening Tag
        assert!(self.consume_char().unwrap() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char().unwrap() == '>');

        // Contents
        // 子コードを解析した結果
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char().unwrap() == '<');
        assert!(self.consume_char().unwrap() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char().unwrap() == '>');

        dom::Node::elem(tag_name, attrs, children)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char().unwrap();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char().unwrap(), open_quote);
        value
    }

    fn parse_attr(&mut self) -> Result<(String, String), error::Error> {
        let name = self.parse_tag_name();
        if self.consume_char()? != '=' {
            return Err(error::Error::ReadError);
        }
        let value = self.parse_attr_value();
        Ok((name, value))
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char().unwrap() == '>' {
                break;
            }
            let (name, value) = self.parse_attr().unwrap();
            attributes.insert(name, value);
        }
        attributes
    }

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

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // 現在の位置の文字列を取得し、位置を進める
    fn consume_char(&mut self) -> Result<char, error::Error> {
        let mut char_indicies = self.source[self.position..].char_indices();
        let (_, cur_char) = char_indicies.next().unwrap();
        let (next_pos, _) = char_indicies.next().unwrap_or((1, ' '));
        self.position += next_pos;
        Ok(cur_char)
    }

    fn next_char(&self) -> Result<char, error::Error> {
        self.source[self.position..]
            .chars()
            .next()
            .ok_or(error::Error::ReadError)
    }

    fn starts_with(&self, s: &str) -> bool {
        self.source[self.position..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.position >= self.source.len()
    }
}
