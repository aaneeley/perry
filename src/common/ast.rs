use super::*;

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Literal(LiteralExpression),
    FunctionCall(Box<FunctionCall>),
    VariableRef(Box<VariableRef>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    // Function(FunctionDecl),
    Variable(VariableDecl),
    If(IfStatement),
    // Return(ReturnStatement),
    Expr(Expression), // To allow void expressions in function body
}

#[derive(Debug, PartialEq)]
pub struct VariableDecl {
    pub name: String,
    pub value: Expression,
    pub type_name: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type_name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub callee: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct VariableRef {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: Expression,
    pub right: Expression,
    pub operator: token::BinaryOperator,
}

#[derive(Debug, PartialEq)]
pub struct LiteralExpression {
    pub value: LiteralValue,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    String(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_body: Vec<Statement>,
    pub else_body: Option<Box<IfStatement>>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Expression,
}
