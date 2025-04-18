use crate::lexer::tokens::{Token, TokenType};
use crate::parser::ast::*;
use crate::parser::ast_builder::ASTBuilder;

pub fn parse(tokens: &[Token]) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut current = 0;

    while current < tokens.len() {
        ast.push(walk(tokens, &mut current));
    }

    // println!("{:?}", ast);
    ast
}

// === Expression Parsing ===

fn parse_expression(tokens: &[Token], current: &mut usize) -> ASTNode {
    parse_additive_expression(tokens, current)
}

fn parse_additive_expression(tokens: &[Token], current: &mut usize) -> ASTNode {
    let mut node = parse_multiplicative_expression(tokens, current);

    while let Some(token) = tokens.get(*current) {
        if token.token_type == TokenType::Operator && (token.value == "+" || token.value == "-") {
            let operator = token.value.clone();
            *current += 1;
            let right = parse_multiplicative_expression(tokens, current);
            node = ASTBuilder::create_binary_expression_node(operator, node, right);
        } else {
            break;
        }
    }

    node
}

fn parse_multiplicative_expression(tokens: &[Token], current: &mut usize) -> ASTNode {
    let mut node = parse_primary_expression(tokens, current);

    while let Some(token) = tokens.get(*current) {
        if token.token_type == TokenType::Operator && (token.value == "*" || token.value == "/") {
            let operator = token.value.clone();
            *current += 1;
            let right = parse_primary_expression(tokens, current);
            node = ASTBuilder::create_binary_expression_node(operator, node, right);
        } else {
            break;
        }
    }

    node
}

fn parse_primary_expression(tokens: &[Token], current: &mut usize) -> ASTNode {
    let token = &tokens[*current];

    match token.token_type {
        TokenType::Number => {
            *current += 1;
            let value = token.value.parse::<f64>().unwrap();
            ASTBuilder::create_number_literal_node(value)
        }
        TokenType::String => {
            *current += 1;
            ASTBuilder::create_string_literal_node(token.value.clone())
        }
        TokenType::Bool => {
            *current += 1;
            if token.value == "twue" {
                ASTBuilder::create_bool_literal_node(1)
            } else {
                ASTBuilder::create_bool_literal_node(0)
            }
        }
        TokenType::Identifier => parse_identifier(tokens, current),
        TokenType::Parenthesis if token.value == "(" => {
            *current += 1;
            let expr = parse_expression(tokens, current);
            expect_parenthesis(tokens, current, ")");
            expr
        }
        TokenType::KindOf => parse_kind_of(tokens, current),
        _ => panic!("Unexpected token in expression: {:?}", token.token_type),
    }
}

// === Statement Parsing ===

fn walk(tokens: &[Token], current: &mut usize) -> ASTNode {
    match tokens[*current].token_type {
        TokenType::Print => parse_print(tokens, current),
        TokenType::OwO => parse_owo(tokens, current),
        TokenType::KindOf => parse_kind_of(tokens, current),
        TokenType::FunctionDef => parse_function_declaration(tokens, current),
        TokenType::VarDecl => parse_variable_declaration(tokens, current),
        TokenType::Identifier => parse_identifier(tokens, current),
        _ => panic!(
            "Unexpected token at top level: {:?}",
            tokens[*current].token_type
        ),
    }
}

fn parse_print(tokens: &[Token], current: &mut usize) -> ASTNode {
    *current += 1;
    expect_parenthesis(tokens, current, "(");
    let expr = parse_expression(tokens, current);
    expect_parenthesis(tokens, current, ")");
    ASTBuilder::create_print_node(expr)
}

fn parse_owo(tokens: &[Token], current: &mut usize) -> ASTNode {
    *current += 1;
    expect_parenthesis(tokens, current, "(");
    let expr = parse_expression(tokens, current);
    expect_parenthesis(tokens, current, ")");
    ASTBuilder::create_owo_node(expr)
}

fn parse_kind_of(tokens: &[Token], current: &mut usize) -> ASTNode {
    *current += 1;
    expect_parenthesis(tokens, current, "(");
    let expr = parse_expression(tokens, current);
    expect_parenthesis(tokens, current, ")");
    ASTBuilder::create_kind_of_node(expr)
}

fn parse_function_declaration(tokens: &[Token], current: &mut usize) -> ASTNode {
    *current += 1;

    let name_token = &tokens[*current];
    if name_token.token_type != TokenType::Identifier {
        panic!("Expected function name after sparkle");
    }

    let name = name_token.value.clone();
    *current += 1;
    expect_parenthesis(tokens, current, "(");

    let mut params = Vec::new();
    while let Some(token) = tokens.get(*current) {
        if token.token_type == TokenType::Parenthesis && token.value == ")" {
            break;
        }

        if token.token_type == TokenType::OwO || token.token_type == TokenType::Print || token.token_type == TokenType::FunctionDef || token.token_type == TokenType::VarDecl || token.token_type == TokenType::Return || token.token_type == TokenType::KindOf {
            panic!("You can't use a keyword as a function parameter.")
        } 
        
        if token.token_type == TokenType::Identifier {
            let param_name = token.value.clone();
            params.push(ASTNode::VariableDeclaration(VariableDeclarationNode {
                name: param_name,
                value: Box::new(ASTNode::None), // Placeholder value
            }));
            *current += 1;
        }
        
        if token.token_type == TokenType::Comma {
            *current += 1; // Skip comma and continue to next parameter
        }
    }

    expect_parenthesis(tokens, current, ")");
    expect_brace(tokens, current, "{");

    let mut body = Vec::new();
    while tokens.get(*current).map_or(false, |t| {
        t.token_type != TokenType::Brace || t.value != "}"
    }) {
        if tokens[*current].token_type == TokenType::Return {
            body.push(parse_return(tokens, current));
        } else {
            body.push(walk(tokens, current));
        }
    }

    *current += 1; // Skip '}'
    ASTBuilder::create_function_declaration_node(name, params, body)
}

fn parse_return(tokens: &[Token], current: &mut usize) -> ASTNode {
    *current += 1; // Skip 'bringback'
    let value = parse_expression(tokens, current);
    ASTBuilder::create_return_node(value)
}

fn parse_variable_declaration(tokens: &[Token], current: &mut usize) -> ASTNode {
    *current += 1;

    let name_token = &tokens[*current];
    if name_token.token_type != TokenType::Identifier {
        panic!("Expected variable name after nyan");
    }

    let name = name_token.value.clone();
    *current += 1;

    expect_token_type(tokens, current, TokenType::Equal);
    let value = parse_expression(tokens, current);
    ASTBuilder::create_variable_declaration_node(name, value)
}

fn parse_identifier(tokens: &[Token], current: &mut usize) -> ASTNode {
    let name = tokens[*current].value.clone();

    if tokens.get(*current + 1).map_or(false, |t| {
        t.token_type == TokenType::Parenthesis && t.value == "("
    }) {
        *current += 2; // Skip the identifier and '('

        let mut arguments = Vec::new();

        // Parse arguments separated by commas
        while let Some(token) = tokens.get(*current) {
            if token.token_type == TokenType::Parenthesis && token.value == ")" {
                break;
            }

            if token.token_type != TokenType::Comma {
                // Only parse expression if not a comma
                arguments.push(parse_expression(tokens, current));
            }

            // Skip commas
            if token.token_type == TokenType::Comma {
                *current += 1;
            }
        }

        expect_parenthesis(tokens, current, ")");
        ASTBuilder::create_function_call_node(name, arguments)
    } else {
        *current += 1;
        ASTBuilder::create_variable_reference_node(name)
    }
}

// === Token Expectations ===

fn expect_token_type(tokens: &[Token], current: &mut usize, expected: TokenType) {
    let token = &tokens[*current];
    if token.token_type != expected {
        panic!(
            "Expected token type {:?}, but got {:?}",
            expected, token.token_type
        );
    }
    *current += 1;
}

fn expect_parenthesis(tokens: &[Token], current: &mut usize, expected: &str) {
    let token = &tokens[*current];
    if token.token_type != TokenType::Parenthesis || token.value != expected {
        panic!("Expected parenthesis '{}'", expected);
    }
    *current += 1;
}

fn expect_brace(tokens: &[Token], current: &mut usize, expected: &str) {
    let token = &tokens[*current];
    if token.token_type != TokenType::Brace || token.value != expected {
        panic!("Expected brace '{}'", expected);
    }
    *current += 1;
}
