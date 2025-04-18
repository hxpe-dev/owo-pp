#[derive(Debug, Clone)]
pub enum ASTNode {
    OwO(OwONode),
    Print(PrintNode),
    FunctionDeclaration(FunctionDeclarationNode),
    FunctionCall(FunctionCallNode),
    VariableDeclaration(VariableDeclarationNode),
    VariableReference(VariableReferenceNode),
    StringLiteral(StringLiteralNode),
    NumberLiteral(NumberLiteralNode),
    BoolLiteral(BoolLiteralNode),
    BinaryExpression(BinaryExpressionNode),
    Return(ReturnNode),
    KindOf(KindOfNode),
    None,
}

#[derive(Debug, Clone)]
pub struct PrintNode {
    pub expression: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct OwONode {
    pub expression: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclarationNode {
    pub name: String,
    pub params: Vec<ASTNode>,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarationNode {
    pub name: String,
    pub value: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct VariableReferenceNode {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct StringLiteralNode {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct NumberLiteralNode {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct BoolLiteralNode {
    pub value: i8,
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub value: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct KindOfNode {
    pub expression: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpressionNode {
    pub operator: String,
    pub left: Box<ASTNode>,
    pub right: Box<ASTNode>,
}
