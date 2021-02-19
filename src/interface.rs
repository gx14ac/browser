use dom::dom::{AttrMap, Node};
use error::Error;

use crate::parser::interface::DefaultParserTrait;

pub trait HTMLParserTrait: DefaultParserTrait {
    fn parse_nodes(&mut self) -> Vec<Node>;

    fn parse_node(&mut self) -> Node;

    fn parse_text(&mut self) -> Node;

    fn parse_element(&mut self) -> Node;

    fn parse_attr(&mut self) -> Result<(String, String), Error>;

    fn parse_attr_value(&mut self) -> String;

    fn parse_attributes(&mut self) -> Result<AttrMap, Error>;

    fn parse_tag_name(&mut self) -> String;
}
