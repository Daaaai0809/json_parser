use crate::{
    lexer::Lexer,
    token::Token,
    node::{Node, SyntaxTree},
    util::PeekableIter
};

pub struct Parser {
    tokens: PeekableIter<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn consume_string(&mut self) -> Node {
        match self.tokens.next() {
            Some(Token::StringValue(string)) => Node::new(SyntaxTree::StringLiteral(string), vec![]),
            Some(invalid_token) => panic!("Invalid token: {:?}", invalid_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_number(&mut self) -> Node {
        match self.tokens.next() {
            Some(Token::NumberValue(number)) => Node::new(SyntaxTree::NumberLiteral(number), vec![]),
            Some(invalid_token) => panic!("Invalid token: {:?}", invalid_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_true(&mut self) -> Node {
        match self.tokens.next() {
            Some(Token::True) => Node::new(SyntaxTree::TrueLiteral, vec![]),
            Some(invalid_token) => panic!("Invalid token: {:?}", invalid_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_false(&mut self) -> Node {
        match self.tokens.next() {
            Some(Token::False) => Node::new(SyntaxTree::FalseLiteral, vec![]),
            Some(invalid_token) => panic!("Invalid token: {:?}", invalid_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_null(&mut self) -> Node {
        match self.tokens.next() {
            Some(Token::Null) => Node::new(SyntaxTree::NullLiteral, vec![]),
            Some(invalid_token) => panic!("Invalid token: {:?}", invalid_token),
            None => panic!("Unexpected end of input"),
        }
    }

    fn consume_object_literal(&mut self) -> Result<Node, String> {
        let mut property_assignments = Vec::new();
        self.tokens.next();
        loop {
            match self.tokens.peek() {
                Some(Token::RBrace) => {
                    self.tokens.next();
                    break;
                }
                Some(Token::StringValue(_)) => match self.consume_property_assignment() {
                    Ok(property_assignment) => property_assignments.push(property_assignment),
                    Err(err) => return Err(err),
                }
                Some(Token::Comma) => {
                    self.tokens.next();
                }
                _ => return Err("Unexpected token".to_string()),
            }
        }

        Ok(Node::new(SyntaxTree::ObjectLiteral, property_assignments))
    }

    fn consume_property_assignment(&mut self) -> Result<Node, String> {
        let identifier = match self.tokens.peek() {
            Some(Token::StringValue(s)) => s.clone(),
            _ => return Err("Unexpected token".to_string()),
        };
        
        self.tokens.next();
        self.tokens.next();

        match self.consume_expression() {
            Ok(val) => Ok(Node::new(
                SyntaxTree::PropertyAssignment, 
                vec![
                    Node::new(SyntaxTree::Identifier(identifier), vec![]),
                    val,
                ],
            )),
            Err(err) => Err(err),
        }
    }

    fn consume_array_literal(&mut self) -> Result<Node, String> {
        let mut elements = Vec::new();

        self.tokens.next();
        
        loop {
            match self.tokens.peek() {
                Some(Token::RBracket) => {
                    self.tokens.next();
                    break;
                }
                Some(Token::Comma) => {
                    self.tokens.next();
                }
                _ => match self.consume_expression() {
                    Ok(element) => elements.push(element),
                    Err(err) => return Err(err),
                }
            }
        }

        Ok(Node::new(SyntaxTree::ArrayLiteral, elements))
    }

    fn consume_expression(&mut self) -> Result<Node, String> {
        match self.tokens.peek() {
            Some(Token::StringValue(_)) => Ok(self.consume_string()),
            Some(Token::NumberValue(_)) => Ok(self.consume_number()),
            Some(Token::True) => Ok(self.consume_true()),
            Some(Token::False) => Ok(self.consume_false()),
            Some(Token::Null) => Ok(self.consume_null()),
            Some(Token::LBrace) => self.consume_object_literal(),
            Some(Token::LBracket) => self.consume_array_literal(),
            _ => Err("Unexpected token".to_string()),
        }
    }

    pub fn parse(&mut self) -> Node {
        let first_token = self.tokens.peek();
        let res = match first_token {
            Some(Token::LBrace) => self.consume_object_literal(),
            Some(Token::LBracket) => self.consume_array_literal(),
            _ => Err("Unexpected token".to_string()),
        };

        match res {
            Ok(res) => res,
            Err(err) => panic!("Parse error: {}", err),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object_literal_success() {
        let input = r#"{"foo": "bar"}"#;
        let mut parser = Parser::new(input);
        let node = parser.parse();
        assert_eq!(
            node,
            Node::new(
                SyntaxTree::ObjectLiteral,
                vec![Node::new(
                    SyntaxTree::PropertyAssignment,
                    vec![
                        Node::new(SyntaxTree::Identifier("foo".to_string()), vec![]),
                        Node::new(SyntaxTree::StringLiteral("bar".to_string()), vec![]),
                    ],
                ),],
            ),
        );
    }

    #[test]
    #[should_panic(expected = "Parse error: Unexpected token")]
    fn test_parse_object_literal_failure() {
        let input = r#"{"foo": "bar""#;
        let mut parser = Parser::new(input);
        let node = parser.parse();
        assert_ne!(
            node,
            Node::new(
                SyntaxTree::ObjectLiteral,
                vec![Node::new(
                    SyntaxTree::PropertyAssignment,
                    vec![
                        Node::new(SyntaxTree::Identifier("foo".to_string()), vec![]),
                        Node::new(SyntaxTree::StringLiteral("bar".to_string()), vec![]),
                    ],
                ),],
            ),
        );
    }

    #[test]
    fn test_parse_array_literal_success() {
        let input = r#"["foo", "bar"]"#;
        let mut parser = Parser::new(input);
        let node = parser.parse();
        assert_eq!(
            node,
            Node::new(
                SyntaxTree::ArrayLiteral,
                vec![
                    Node::new(SyntaxTree::StringLiteral("foo".to_string()), vec![]),
                    Node::new(SyntaxTree::StringLiteral("bar".to_string()), vec![]),
                ],
            ),
        );
    }

    #[test]
    #[should_panic(expected = "Parse error: Unexpected token")]
    fn test_parse_array_literal_failure() {
        let input = r#"["foo", "bar""#;
        let mut parser = Parser::new(input);
        let node = parser.parse();
        assert_ne!(
            node,
            Node::new(
                SyntaxTree::ArrayLiteral,
                vec![
                    Node::new(SyntaxTree::StringLiteral("foo".to_string()), vec![]),
                    Node::new(SyntaxTree::StringLiteral("bar".to_string()), vec![]),
                ],
            ),
        );
    }
}
