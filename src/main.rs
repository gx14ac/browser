use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn create_node_with_text(node_text: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(node_text),
    }
}

fn create_node_with_attrs(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: tag_name,
            attributes: attrs,
        }),
    }
}

fn main() {
    println!("Hello, world!");
}
