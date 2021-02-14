// HTML Parser
use dom;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pos: usize,    // 文字列位置
    input: String, // 入力文字列
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input: input.to_string(),
        }
    }

    /*
        HTMLドキュメント全体を解析して DOM ツリーにする
        ドキュメントにルートノードが明示的に含まれていない場合、そのドキュメントのルートノードを作成します。
    */
    fn parse(source: String) -> dom::Node {
        let mut nodes = match Parser::new(source).parse_nodes() {
            Ok(nodes) => nodes,
            Err(_) => panic!("parse nodes error"),
        };

        // ルート要素が含まれている場合はそのまま返す <html>など
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            // ルート要素が含まれていない場合は作成する
            dom::Node::elem("html".to_string(), HashMap::new(), nodes)
        }
    }

    /*
        子ノードを解析するために、終了タグに到達するまでループ内で parse_node を再帰的に呼び出す
    */
    fn parse_nodes(&mut self) -> Result<Vec<dom::Node>, ()> {
        let mut nodes: Vec<dom::Node> = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            if let Ok(node) = self.parse_node() {
                nodes.push(node);
            }
        }
        Ok(nodes)
    }

    /*
        要素かテキストノードかをハンドリングし、各パースを実行する
    */
    fn parse_node(&mut self) -> Result<dom::Node, ()> {
        match self.next_char()? {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /*
        TextNodeをパース
    */
    // '<' 以外の文字を返す
    fn parse_text(&mut self) -> Result<dom::Node, ()> {
        Ok(dom::Node::text(self.consume_while(|c| c != '<')?))
    }

    /*
        Elementをパース
    */

    /*
        要素には開閉タグ<>とその間に任意の数の子ノードが含まれる
    */

    fn parse_element(&mut self) -> Result<dom::Node, ()> {
        // Opening Tag
        assert!(self.consume_char()? == '<');
        let tag_name = self.parse_tag_name()?;
        let attrs = self.parse_attributes()?;
        assert!(self.consume_char()? == '>');

        // Contents
        // 子コードを解析した結果
        let children = self.parse_nodes()?;

        // Closing tag.
        assert!(self.consume_char()? == '<');
        assert!(self.consume_char()? == '/');
        assert!(self.parse_tag_name()? == tag_name);
        assert!(self.consume_char()? == '>');

        return Ok(dom::Node::elem(tag_name, attrs, children));
    }

    /*
        属性をパースするための関数群
        属性は color=red とかそういうの
        colorをname
        redをvalue
        として扱う
        https://developer.mozilla.org/ja/docs/Web/HTML/Attributes
    */

    /*
        name="value" のペアを返す
    */
    fn parse_attr(&mut self) -> Result<(String, String), ()> {
        let name = self.parse_tag_name()?;
        if self.consume_char()? == '=' {
            return Err(());
        }

        let value = self.parse_attr_value()?;
        Ok((name, value))
    }

    /*
        引用符で囲まれた値を返す
    */
    fn parse_attr_value(&mut self) -> Result<String, ()> {
        let open_quote = self.consume_char()?;
        self.consume_while(|c| c != open_quote)
    }

    /*
        属性をパースし、返す
        オープニングタグ(>)の最後に到達するまで、名前の後に=が続き、引用符で囲まれた文字列を繰り返し探しています。
    */
    fn parse_attributes(&mut self) -> Result<dom::AttrMap, ()> {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char()? == '>' {
                break;
            }
            match self.parse_attr() {
                Ok(x) => {
                    attributes.insert(x.0, x.1);
                }
                Err(()) => {}
            }
        }
        Ok(attributes)
    }

    // 与えられた条件を満たす文字を消費し、文字列として返す
    // 引数は char を受け取り、bool を返す関
    fn consume_while<F>(&mut self, f: F) -> Result<String, ()>
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        // eofでない && next_charが指定された条件を満たす文字なら繰り返す
        while !self.eof() && f(self.next_char()?) {
            // posを進める && resultにpushする
            result.push(self.consume_char()?);
        }
        Ok(result)
    }

    fn consume_whitespace(&mut self) -> Result<(), ()> {
        self.consume_while(char::is_whitespace).and(Ok(()))
    }

    // 英数字のみだけ返す
    fn parse_tag_name(&mut self) -> Result<String, ()> {
        self.consume_while(|c| c.is_alphanumeric())
    }

    // 現在の文字列を取得し、posを1つ進める
    fn consume_char(&mut self) -> Result<char, ()> {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(())?;
        // for do not panic.
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        Ok(cur_char)
    }

    // inputに対してposのpositionから次の文字を取り出す
    fn next_char(&self) -> Result<char, ()> {
        self.input[self.pos..].chars().next().ok_or(())
    }

    // posの位置にある接頭辞がsとマッチするか
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // 全ての文字列を対象とたかを確認
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

#[test]
fn test_consume_char() {
    let src = "令和";
    let mut parser = Parser::new(src.to_string());

    let consume_char = parser.consume_char().unwrap();
    assert_eq!(consume_char.to_string(), "令");
}

#[test]
fn test_parse() {
    let src = "<html><body>Hello, world!</body></html>";
    let dom_node = Parser::parse(src.to_string());
    println!("{}", dom_node);
}
