use parser::interface::DefaultParserTrait;
use style_sheet::declaration::Declaration;
use style_sheet::rule::Rule;
use style_sheet::selector::{Selector, Specificity};
use style_sheet::simple_selector::SimpleSelector;
use style_sheet::util::{Unit, Value};

pub trait SelectorTrait {
    fn specificity(&self) -> Specificity;
}

pub trait RuleTrait {}

pub trait ColorTrait {}

pub trait ValueTrait {
    fn to_px(&self) -> f32;
}

pub trait CSSParserTrait: DefaultParserTrait {
    fn parse_rules(&mut self) -> Vec<Rule>;
    fn parse_rule(&mut self) -> Rule;
    fn parse_simple_selector(&mut self) -> SimpleSelector;
    fn parse_selector(&mut self) -> Vec<Selector>;
    fn parse_declarations(&mut self) -> Vec<Declaration>;
    fn parse_declaration(&mut self) -> Declaration;
    fn parse_value(&mut self) -> Value;
    fn parse_length(&mut self) -> Value;
    fn parse_float(&mut self) -> f32;
    fn parse_unit(&mut self) -> Unit;
    fn parse_color(&mut self) -> Value;
    fn parse_hex_pair(&mut self) -> u8;
    fn parse_identifier(&mut self) -> String;
}
