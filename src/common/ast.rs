use super::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Program {
    pub body: Vec<SpannedStatement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WithSpan<T> {
    pub node: T,
    pub span: Span,
}

pub trait Spannable: Sized {
    fn spanned(self, span: Span) -> WithSpan<Self> {
        WithSpan { node: self, span }
    }
}

impl<T> WithSpan<T> {
    pub fn as_ref(&self) -> &T {
        &self.node
    }
}

pub type SpannedStatement = WithSpan<Statement>;
pub type SpannedExpression = WithSpan<Expression>;
impl Spannable for Statement {}
impl Spannable for Expression {}

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
    pub value: SpannedExpression,
    pub type_name: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type_name: String,
    pub body: Vec<SpannedStatement>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub callee: String,
    pub args: Vec<SpannedExpression>,
}

#[derive(Debug, PartialEq)]
pub struct VariableRef {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: SpannedExpression,
    pub right: SpannedExpression,
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
    pub condition: SpannedExpression,
    pub then_body: Vec<SpannedStatement>,
    pub else_body: Option<Box<IfStatement>>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: SpannedExpression,
}
