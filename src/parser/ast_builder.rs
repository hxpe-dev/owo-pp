use crate::parser::ast::*;

pub struct ASTBuilder;

impl ASTBuilder {
    pub fn create_print_node(expression: ASTNode) -> ASTNode {
        ASTNode::Print(PrintNode {
            expression: Box::new(expression),
        })
    }

    pub fn create_owo_node(expression: ASTNode) -> ASTNode {
        ASTNode::OwO(OwONode {
            expression: Box::new(expression),
        })
    }

    pub fn create_function_declaration_node(
        name: String,
        params: Vec<ASTNode>,
        body: Vec<ASTNode>,
    ) -> ASTNode {
        ASTNode::FunctionDeclaration(FunctionDeclarationNode { name, params, body })
    }

    pub fn create_variable_declaration_node(name: String, value: ASTNode) -> ASTNode {
        ASTNode::VariableDeclaration(VariableDeclarationNode {
            name,
            value: Box::new(value),
        })
    }

    pub fn create_binary_expression_node(
        operator: String,
        left: ASTNode,
        right: ASTNode,
    ) -> ASTNode {
        ASTNode::BinaryExpression(BinaryExpressionNode {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn create_variable_reference_node(name: String) -> ASTNode {
        ASTNode::VariableReference(VariableReferenceNode { name })
    }

    pub fn create_string_literal_node(value: String) -> ASTNode {
        ASTNode::StringLiteral(StringLiteralNode { value })
    }

    pub fn create_number_literal_node(value: f64) -> ASTNode {
        ASTNode::NumberLiteral(NumberLiteralNode { value })
    }

    pub fn create_function_call_node(name: String, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::FunctionCall(FunctionCallNode { name, arguments })
    }

    pub fn create_return_node(value: ASTNode) -> ASTNode {
        ASTNode::Return(ReturnNode {
            value: Box::new(value),
        })
    }
}
