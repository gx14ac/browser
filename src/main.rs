extern crate browser;

use std::default::Default;
use std::fs::File;
use std::io::{BufWriter, Read};

use browser::{html_parser, interface::HtmlParserTrait};

fn main() {
    let html = read_source("test.html".to_string());
    // let css = read_source("test.css".to_string());

    let root_node = browser::html_parser::NewHtmlParser(html).parse_nodes();
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
