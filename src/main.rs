extern crate browser;

use std::default::Default;
use std::fs::File;
use std::io::{BufWriter, Read};

fn main() {
    let html = read_source("test.html".to_string());
    // let css = read_source("test.css".to_string());

    let root_node = browser::html::parse(html);

}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
