use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    OwO,
    Print,
    FunctionDef,
    VarDecl,
    Identifier,
    Number,
    String,
    Operator,
    Parenthesis,
    Brace,
    Comma,
    Equal,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn create_token(token_type: TokenType, value: &str) -> Token {
    Token {
        token_type,
        value: value.to_string(),
    }
}

pub fn keywords() -> HashMap<&'static str, TokenType> {
    let mut map = HashMap::new();
    map.insert("owo", TokenType::OwO);
    map.insert("meow", TokenType::Print);
    map.insert("sparkle", TokenType::FunctionDef);
    map.insert("nyan", TokenType::VarDecl);
    map.insert("bringback", TokenType::Return);
    map
}
