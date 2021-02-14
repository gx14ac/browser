//DOM Data Structure

use std::collections::HashMap;
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Node {
    pub fn text(data: String) -> Node {
        Node {
            children: vec![],
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children: children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
}

pub fn create_node_with_text(node_text: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(node_text),
    }
}

pub fn create_node_with_attrs(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: tag_name,
            attributes: attrs,
        }),
    }
}

#[test]
fn test_id() {
    assert_eq!(
        ElementData {
            tag_name: "".to_string(),
            attributes: HashMap::new(),
        }
        .id(),
        None
    )
}
