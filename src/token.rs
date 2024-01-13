use phf::phf_map;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 記号
    LBrace, // {
    RBrace, // }
    LBracket, // [
    RBracket, // ]
    Colon, // :
    Comma, // ,
    // リテラル
    StringValue(String),
    NumberValue(f64),
    True,
    False,
    Null,
    // 終端
    End,
}

pub static CHAR_TOKENS: phf::Map<char, Token> = phf_map! {
    '{' => Token::LBrace,
    '}' => Token::RBrace,
    '[' => Token::LBracket,
    ']' => Token::RBracket,
    ':' => Token::Colon,
    ',' => Token::Comma,
};

pub static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "true" => Token::True,
    "false" => Token::False,
    "null" => Token::Null,
};
