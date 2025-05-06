#![allow(dead_code)]

pub enum Expression {
    Binary(Box<BinaryExpression>),
    Literal(LiteralExpression),
    FunctionCall(Box<FunctionCall>),
    VariableRef(Box<VariableRef>),
}

pub enum Statement {
    Function(FunctionDecl),
    Variable(VariableDecl),
    If(IfStatement),
    Return(ReturnStatement),
    Expr(Expression), // To allow void expressions in function body
}

pub struct VariableDecl {
    name: String,
    value: Expression,
}

pub struct FunctionDecl {
    name: String,
    params: Vec<String>, // TODO: Make this a struct with type
    body: Vec<Statement>,
}

pub struct FunctionCall {
    callee: Box<Expression>,
    args: Vec<Expression>,
}

pub struct VariableRef {
    name: String,
}

pub struct BinaryExpression {
    left: Expression,
    right: Expression,
    operator: BinaryOperator,
}

pub struct LiteralExpression {
    pub value: LiteralValue,
}

pub enum LiteralValue {
    String(String),
    Number(i32),
    Bool(bool),
}

pub enum BinaryOperator {
    Add,
    Subtract,
}

pub struct IfStatement {
    condition: Expression,
    then_body: Vec<Statement>,
    else_body: Vec<Statement>,
}

pub struct ReturnStatement {
    value: Expression,
}
