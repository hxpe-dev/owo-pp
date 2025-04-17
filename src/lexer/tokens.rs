use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Print,
    FunctionDef,
    VarDecl,
    Identifier,
    Number,
    String,
    Operator,
    Parenthesis,
    Brace,
    Equal,
    Return,
}

/* impl TokenType {
    // Optional: A method to return a string representation of the enum variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Print => "PRINT",
            TokenType::FunctionDef => "FUNCTION_DEF",
            TokenType::VarDecl => "VAR_DECL",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::Number => "NUMBER",
            TokenType::String => "STRING",
            TokenType::Operator => "OPERATOR",
            TokenType::Parenthesis => "PARENTHESIS",
            TokenType::Brace => "BRACE",
            TokenType::Equal => "EQUAL",
        }
    }
} */

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
    map.insert("meow", TokenType::Print);
    map.insert("sparkle", TokenType::FunctionDef);
    map.insert("nyan", TokenType::VarDecl);
    map.insert("bringback", TokenType::Return);
    map
}
