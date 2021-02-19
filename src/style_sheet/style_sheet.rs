use style_sheet::css_parser::*;
use style_sheet::rule::Rule;

use super::interface::CSSParserTrait;

#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub fn new_style_sheet(source: String) -> Stylesheet {
    Stylesheet {
        rules: new_css_parser(source).parse_rules(),
    }
}
