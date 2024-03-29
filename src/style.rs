// domにcssスタイルシートを適用するファイル
use dom::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;
use style_sheet::interface::SelectorTrait;
use style_sheet::rule::Rule;
use style_sheet::selector::{Selector, Specificity};
use style_sheet::simple_selector::SimpleSelector;
use style_sheet::style_sheet::Stylesheet;
use style_sheet::util::Value;

type CSSPropertyMap = HashMap<String, Value>;

pub enum Display {
    Inline,
    Block,
    None,
}

#[derive(Debug)]
/*
 各タグに対応したのcssの値を取得する
 node: domのnode,
 specified_values: cssのプロパティをKeyValue配列
 children: 次のタグのStyledNode
*/
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub css_properties: CSSPropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.css_properties.get(name).map(|v| v.clone())
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

// cssで定義されているセレクターとdomのchildrenの各要素のタグの整合性の確認
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // tagチェック
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // id チェック
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // class チェック
    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        css_properties: match root.node_type {
            NodeType::Element(ref elem) => parse_css_property(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

fn parse_css_property(elem: &ElementData, stylesheet: &Stylesheet) -> CSSPropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}
