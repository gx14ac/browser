// HTML Parser
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

    // 現在の文字列を取得し、posを1つ進める
    fn consumer_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        // for do not panic.
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        return cur_char;
    }

    // inputに対してposのpositionから次の文字を取り出す
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
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
fn test() {
    let src = "令和";
    let mut parser = Parser::new(src.to_string());

    let consumer_char = parser.consumer_char();
    assert_eq!(consumer_char.to_string(), "令");
}
