use std::{fmt::Display, str::FromStr};

use super::*;

#[derive(Debug, Clone)]
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

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

pub type SpannedStatement = WithSpan<Statement>;
pub type SpannedExpression = WithSpan<Expression>;
impl Spannable for Statement {}
impl Spannable for Expression {}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Void,
    Bool,
    Int,
    String,
    // Array(Box<Type>),
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Type::Int),
            "bool" => Ok(Type::Bool),
            "string" => Ok(Type::String),
            "void" => Ok(Type::Void),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Literal(LiteralExpression),
    FunctionCall(Box<FunctionCall>),
    VariableRef(Box<VariableRef>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Function(FunctionDecl),
    VarAssignment(VariableAssignment),
    VarDecl(VariableDecl),
    If(IfStatement),
    Loop(LoopStatement),
    Return(ReturnStatement),
    Expr(Expression), // To allow void expressions in function body
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssignment {
    pub name: String,
    pub value: SpannedExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub value: SpannedExpression,
    pub type_: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Parameter>,
    pub type_: Type,
    pub body: Vec<SpannedStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub callee: String,
    pub args: Vec<SpannedExpression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableRef {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: SpannedExpression,
    pub right: SpannedExpression,
    pub operator: token::BinaryOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LiteralExpression {
    pub value: LiteralValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    String(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: SpannedExpression,
    pub then_body: Vec<SpannedStatement>,
    pub else_body: Option<Box<IfStatement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopStatement {
    pub condition: SpannedExpression,
    pub body: Vec<SpannedStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub value: SpannedExpression,
}
