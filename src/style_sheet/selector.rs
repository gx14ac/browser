use style_sheet::interface::SelectorTrait;
use style_sheet::simple_selector::SimpleSelector;

pub type Specificity = (usize, usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

impl SelectorTrait for Selector {
    fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let id_count = simple.id.iter().count();
        let class_len = simple.class.len();
        let tag_count = simple.tag_name.iter().count();
        (id_count, class_len, tag_count)
    }
}
