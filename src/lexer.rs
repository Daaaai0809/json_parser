use crate::{
    token::{Token, CHAR_TOKENS, KEYWORDS},
    util::PeekableIter
};

pub struct Lexer {
    chars: PeekableIter<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let vec: Vec<char> = input.chars().collect();
        Lexer {
            chars: vec.into_iter().peekable(),
        }
    }

    fn consume_char(&mut self) -> Token {
        match self.chars.next() {
            Some(c) => match CHAR_TOKENS.get(&c) {
                Some(token) => token.clone(),
                None => panic!("Unexpected character: {}", c),
            }
            None => panic!("Unexpected character"),
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some(c) if c.is_whitespace() => {
                    self.chars.next();
                }
                _ => break,
            }
        }
    }

    fn consume_string(&mut self) -> Token {
        if self.chars.peek() == Some(&'"') {
            self.chars.next();
        }
        let mut string = String::new();
        loop {
            match self.chars.next() {
                Some('"') => break,
                Some(c) => {
                    string.push(c);
                }
                None => panic!("unexpected token"),
            }
        }
        Token::StringValue(string)
    }

    fn consume_number(&mut self) -> Token {
        let mut number = String::new();
        loop {
            match self.chars.peek() {
                Some(c) if c.is_numeric() || c == &'.'  => match self.chars.next() {
                    Some(c) => number.push(c),
                    None => break,
                }
                _ => break,
            }
        }
        match number.parse::<f64>() {
            Ok(n) => Token::NumberValue(n),
            Err(_) => panic!("unexpected token"),
        }
    }

    fn consume_keyword(&mut self) -> Token {
        let mut keyword = String::new();
        loop {
            match self.chars.peek() {
                Some(c) => if c.is_alphabetic() {
                    keyword.push(*c);
                    self.chars.next();
                } else {
                    break;
                }
                None => break,
            }
        }
        match KEYWORDS.get(keyword.as_str()) {
            Some(token) => token.clone(),
            None => panic!("unexpected token"),
        }
    }

    fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        let c = self.chars.peek();
        match c {
            Some(c) => match c {
                '{' | '}' | '[' | ']' | ':' | ',' => self.consume_char(),
                '"' => self.consume_string(),
                '0'..='9' => self.consume_number(),
                'a'..='z' | 'A'..='Z' => self.consume_keyword(),
                _ => panic!("unexpected token"),
            }
            None => Token::End,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.consume_whitespace();
            let token = self.next_token();
            tokens.push(token.clone());
            if token == Token::End {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_char() {
        let mut lexer = Lexer::new("{");
        assert_eq!(lexer.consume_char(), Token::LBrace);
        let mut lexer = Lexer::new("}");
        assert_eq!(lexer.consume_char(), Token::RBrace);
        let mut lexer = Lexer::new("[");
        assert_eq!(lexer.consume_char(), Token::LBracket);
        let mut lexer = Lexer::new("]");
        assert_eq!(lexer.consume_char(), Token::RBracket);
        let mut lexer = Lexer::new(":");
        assert_eq!(lexer.consume_char(), Token::Colon);
        let mut lexer = Lexer::new(",");
        assert_eq!(lexer.consume_char(), Token::Comma);
    }

    #[test]
    fn test_consume_string() {
        let mut lexer = Lexer::new(r#""foo""#);
        assert_eq!(lexer.consume_string(), Token::StringValue("foo".to_string()));
    }

    #[test]
    fn test_consume_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.consume_number(), Token::NumberValue(123.0));
    }

    #[test]
    fn test_consume_keyword() {
        let mut lexer = Lexer::new("true");
        assert_eq!(lexer.consume_keyword(), Token::True);
        let mut lexer = Lexer::new("false");
        assert_eq!(lexer.consume_keyword(), Token::False);
        let mut lexer = Lexer::new("null");
        assert_eq!(lexer.consume_keyword(), Token::Null);
    }

    #[test]
    fn test_consume_whitespace() {
        let mut lexer = Lexer::new("  \t\n\r");
        lexer.consume_whitespace();
        assert_eq!(lexer.chars.peek(), None);
    }

    #[test]
    fn test_next_token() {
        let input = r#"{"foo": "bar"}"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::StringValue("foo".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::StringValue("bar".to_string()));
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::End);
    }

    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::new(r#"
            {
                "foo": "bar",
                "baz": 123,
                "qux": true,
                "quux": false,
                "corge": null,
                "grault": {
                    "garply": "waldo",
                    "fred": 42,
                    "plugh": true,
                    "xyzzy": false,
                    "thud": null,
                    "emptyObject": {},
                    "emptyArray": []
                },
                "emptyObject": {},
                "emptyArray": []
            }
        "#);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::LBrace,
                Token::StringValue("foo".to_string()),
                Token::Colon,
                Token::StringValue("bar".to_string()),
                Token::Comma,
                Token::StringValue("baz".to_string()),
                Token::Colon,
                Token::NumberValue(123.0),
                Token::Comma,
                Token::StringValue("qux".to_string()),
                Token::Colon,
                Token::True,
                Token::Comma,
                Token::StringValue("quux".to_string()),
                Token::Colon,
                Token::False,
                Token::Comma,
                Token::StringValue("corge".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::StringValue("grault".to_string()),
                Token::Colon,
                Token::LBrace,
                Token::StringValue("garply".to_string()),
                Token::Colon,
                Token::StringValue("waldo".to_string()),
                Token::Comma,
                Token::StringValue("fred".to_string()),
                Token::Colon,
                Token::NumberValue(42.0),
                Token::Comma,
                Token::StringValue("plugh".to_string()),
                Token::Colon,
                Token::True,
                Token::Comma,
                Token::StringValue("xyzzy".to_string()),
                Token::Colon,
                Token::False,
                Token::Comma,
                Token::StringValue("thud".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::StringValue("emptyObject".to_string()),
                Token::Colon,
                Token::LBrace,
                Token::RBrace,
                Token::Comma,
                Token::StringValue("emptyArray".to_string()),
                Token::Colon,
                Token::LBracket,
                Token::RBracket,
                Token::RBrace,
                Token::Comma,
                Token::StringValue("emptyObject".to_string()),
                Token::Colon,
                Token::LBrace,
                Token::RBrace,
                Token::Comma,
                Token::StringValue("emptyArray".to_string()),
                Token::Colon,
                Token::LBracket,
                Token::RBracket,
                Token::RBrace,
                Token::End,
            ]
        );   
    }
}

