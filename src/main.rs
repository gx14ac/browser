extern crate browser;

use browser::{
    html::interface::HTMLParserTrait,
    layout::{layout_tree, Dimensions},
    style_sheet::interface::CSSParserTrait,
};

use std::default::Default;
use std::fs::File;
use std::io::Read;

fn main() {
    let html = read_source("test.html".to_string());
    let css = read_source("test.css".to_string());

    let root_node = browser::html::html_parser::new_html_parser(html).parse();
    let stylesheet = browser::style_sheet::css_parser::new_css_parser(css).parse();
    let style_root = browser::style::style_tree(&root_node, &stylesheet);

    let mut dimensions: Dimensions = Default::default();
    dimensions.content.width = 800.0;
    dimensions.content.height = 600.0;

    let layout_tree = layout_tree(&style_root, dimensions);

    println!("{:?}", layout_tree);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
