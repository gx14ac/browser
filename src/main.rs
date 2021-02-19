extern crate browser;

use std::fs::File;
use std::io::Read;

use browser::interface::HTMLParserTrait;

fn main() {
    let html = read_source("test.html".to_string());
    let css = read_source("test.css".to_string());

    let root_node = browser::html_parser::new_html_parser(html).parse_nodes();
    let root_node = browser::style_sheet::css_parser::new_css_parser(css);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
