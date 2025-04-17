use crate::lexer::tokens::{Token, TokenType, create_token, keywords};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut current = 0;
    let keywords_map = keywords();

    while current < chars.len() {
        let ch = chars[current];

        // === Whitespace ===
        if ch.is_whitespace() {
            current += 1;
            continue;
        }

        // === String Literals ===
        if ch == '"' {
            current += 1;
            let mut value = String::new();

            while current < chars.len() && chars[current] != '"' {
                value.push(chars[current]);
                current += 1;
            }

            tokens.push(create_token(TokenType::String, &value));
            current += 1; // Skip closing quote
            continue;
        }

        // === Single-Character Tokens ===
        match ch {
            '(' | ')' => {
                tokens.push(create_token(TokenType::Parenthesis, &ch.to_string()));
                current += 1;
                continue;
            }
            '{' | '}' => {
                tokens.push(create_token(TokenType::Brace, &ch.to_string()));
                current += 1;
                continue;
            }
            ',' => {
                tokens.push(create_token(TokenType::Comma, &ch.to_string()));
                current += 1;
                continue;
            }
            '=' => {
                tokens.push(create_token(TokenType::Equal, "="));
                current += 1;
                continue;
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(create_token(TokenType::Operator, &ch.to_string()));
                current += 1;
                continue;
            }
            _ => {}
        }

        // === Numbers ===
        if ch.is_ascii_digit() {
            let mut num_str = String::new();

            while current < chars.len()
                && (chars[current].is_ascii_digit() || chars[current] == '.')
            {
                num_str.push(chars[current]);
                current += 1;
            }

            tokens.push(create_token(TokenType::Number, &num_str));
            continue;
        }

        // === Identifiers and Keywords ===
        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut value = String::new();

            while current < chars.len()
                && (chars[current].is_alphanumeric() || chars[current] == '_')
            {
                value.push(chars[current]);
                current += 1;
            }

            let token_type = keywords_map
                .get(value.as_str())
                .cloned()
                .unwrap_or(TokenType::Identifier);

            tokens.push(create_token(token_type, &value));
            continue;
        }

        panic!("Unexpected character: {}", ch);
    }

    tokens
}
