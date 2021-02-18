// style => 2次元空間の長方形の束に変換

use std::default::Default;
use style::StyledNode;

enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

/*
   - https://www.w3.org/TR/CSS2/box.html#box-dimensions
   ボックスモデルのコンテンツ領域の構造体
*/
#[derive(Default)]
pub struct Dimensions {
    // ボックスの位置
    content: Rect,

    // 周りを取り囲む値
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Default)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct LayoutBox<'a> {
    dimenstions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            dimenstions: Default::default(),
            box_type: box_type,
            children: Vec::new(),
        }
    }

    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymousBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // ルートの作成
    let mut root = LayoutBox::new(match style_node.display() {
        Block => BoxType::BlockNode(style_node),
        Inline => BoxType::InlineNode(style_node),
        DisplayNone => panic!("Root node has display: none."),
    });

    for child in &style_node.children {
        match child.display() {
            /*
             block
             block
             block
            */
            Block => root.children.push(build_layout_tree(child)),
            /*
             inline inline inline
            */
            Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            DisplayNone => panic!("Root node has display: none."),
        }
    }

    return root;
}
