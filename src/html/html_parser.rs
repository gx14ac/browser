use dom::dom::{AttrMap, Node};
use error::Error;
use html::interface::HTMLParserTrait;
use parser::interface::DefaultParserTrait;
use parser::Parser;
use std::collections::HashMap;

pub fn new_html_parser(source: String) -> impl HTMLParserTrait {
    return Parser {
        position: 0,
        source: source,
    };
}

impl HTMLParserTrait for Parser {
    fn parse(&mut self) -> Node {
        let mut nodes = Parser {
            position: 0,
            source: self.source.clone(),
        }
        .parse_nodes();

        // If the document contains a root element, just return it. Otherwise, create one.
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            Node::elem("html".to_string(), HashMap::new(), nodes)
        }
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    fn parse_node(&mut self) -> Node {
        match self.next_char().unwrap() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Node {
        Node::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> Node {
        // Opening Tag
        assert!(self.consume_char().unwrap() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes().unwrap();
        assert!(self.consume_char().unwrap() == '>');

        // Contents
        // 子コードを解析した結果
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char().unwrap() == '<');
        assert!(self.consume_char().unwrap() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char().unwrap() == '>');

        Node::elem(tag_name, attrs, children)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char().unwrap();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char().unwrap(), open_quote);
        value
    }

    fn parse_attr(&mut self) -> Result<(String, String), Error> {
        let name = self.parse_tag_name();
        if self.consume_char()? != '=' {
            return Err(Error::ReadError);
        }
        let value = self.parse_attr_value();
        Ok((name, value))
    }

    fn parse_attributes(&mut self) -> Result<AttrMap, Error> {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char().unwrap() == '>' {
                break;
            }

            match self.parse_attr() {
                Ok((name, value)) => {
                    attributes.insert(name, value);
                }
                Err(err) => {
                    panic!("{:?}", err);
                }
            }
        }
        Ok(attributes)
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }
}

#[test]
fn test_html_parser() {
    let src = "<html><body><head>aa</head></body></html>";
    let node = new_html_parser(src.to_string()).parse();
    assert_eq!(
        node,
        Node::elem(
            "html".to_string(),
            HashMap::new(),
            vec![Node::elem(
                "body".to_string(),
                HashMap::new(),
                vec![Node::elem(
                    "head".to_string(),
                    HashMap::new(),
                    vec![Node::text("aa".to_string())],
                ),],
            ),],
        ),
    );
}
