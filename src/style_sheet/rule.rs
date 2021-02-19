use style_sheet::declaration::Declaration;
use style_sheet::selector::Selector;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}
