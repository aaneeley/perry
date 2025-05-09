//pub mod test;

use std::{collections::HashMap, fmt::Display, hash::Hash};

use crate::common::{
    ast::{
        self, Expression, FunctionCall, LiteralExpression, LiteralValue, Parameter, Span,
        Spannable, SpannedExpression, SpannedStatement, Statement, Type,
    },
    token::BinaryOperator,
};

#[derive(Clone)]
struct ScopeStack {
    tables: Vec<Vec<SpannedStatement>>,
}

impl ScopeStack {
    fn new() -> ScopeStack {
        ScopeStack { tables: Vec::new() }
    }

    // Add a statement to the current scope
    fn add_statement(&mut self, statement: SpannedStatement) {
        self.tables[0].push(statement);
    }

    fn enter_scope(&mut self) {
        self.tables.insert(0, Vec::new());
    }

    fn exit_scope(&mut self) {
        self.tables.remove(0);
    }

    fn lookup(&self, name: &str) -> Option<&SpannedStatement> {
        self.tables
            .iter()
            .rev()
            .flat_map(|table| table.iter())
            .find(|statement| match statement.as_ref() {
                Statement::VarDecl(var_decl) => var_decl.name == name,
                Statement::Function(func) => func.name == name,
                _ => false,
            })
    }
}

#[derive(Clone)]
pub struct Interpreter<'a> {
    scope_stack: ScopeStack,
    program_ast: &'a ast::Program,
}

impl<'a> Interpreter<'a> {
    pub fn new(program_ast: &'a ast::Program) -> Self {
        Self {
            scope_stack: ScopeStack::new(),
            program_ast,
        }
    }

    fn scope_builtins(&mut self) {
        self.scope_stack.add_statement(
            ast::Statement::Function(ast::FunctionDecl {
                name: "print".to_string(),
                params: vec![ast::Parameter {
                    name: "arg".to_string(),
                    type_: Type::String,
                }],
                type_: Type::Void,
                body: vec![],
            })
            .spanned(Span { line: 1, column: 1 }),
        );
        self.scope_stack.add_statement(
            ast::Statement::Function(ast::FunctionDecl {
                name: "println".to_string(),
                params: vec![ast::Parameter {
                    name: "arg".to_string(),
                    type_: Type::String,
                }],
                type_: Type::Void,
                body: vec![],
            })
            .spanned(Span { line: 1, column: 1 }),
        );
        self.scope_stack.add_statement(
            ast::Statement::Function(ast::FunctionDecl {
                name: "to_str".to_string(),
                params: vec![ast::Parameter {
                    name: "arg".to_string(),
                    type_: Type::Int,
                }],
                type_: Type::String,
                body: vec![],
            })
            .spanned(Span { line: 1, column: 1 }),
        );
    }

    fn evaluate_expression(
        &mut self,
        expression: &ast::SpannedExpression,
    ) -> Result<ast::LiteralValue, RuntimeError> {
        match expression.node.clone() {
            ast::Expression::Literal(literal) => Ok(literal.value),
            ast::Expression::FunctionCall(function_call) => {
                let function = {
                    let name = &function_call.callee;
                    self.scope_stack.lookup(name).cloned().ok_or_else(|| {
                        RuntimeError::new(
                            format!("use of undefined function {}", name),
                            expression.span,
                        )
                    })?
                };

                return self.execute_function(&function, &function_call.args);
            }
            ast::Expression::Binary(binary_expression) => {
                let left_value = self.evaluate_expression(&binary_expression.left)?;
                let right_value = self.evaluate_expression(&binary_expression.right)?;
                match binary_expression.operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::LessThan
                    | BinaryOperator::GreaterThanOrEqual
                    | BinaryOperator::LessThanOrEqual => {
                        let LiteralValue::Number(left_number) = left_value else {
                            return Err(RuntimeError::new(
                                format!("not a number: {}", left_value),
                                binary_expression.left.span,
                            ));
                        };
                        let LiteralValue::Number(right_number) = right_value else {
                            return Err(RuntimeError::new(
                                format!("not a number: {}", right_value),
                                binary_expression.right.span,
                            ));
                        };
                        match binary_expression.operator {
                            BinaryOperator::Add => {
                                Ok(LiteralValue::Number(left_number + right_number))
                            }
                            BinaryOperator::Subtract => {
                                Ok(LiteralValue::Number(left_number - right_number))
                            }
                            BinaryOperator::Multiply => {
                                Ok(LiteralValue::Number(left_number * right_number))
                            }
                            BinaryOperator::Divide => {
                                Ok(LiteralValue::Number(left_number / right_number))
                            }
                            BinaryOperator::Modulo => {
                                Ok(LiteralValue::Number(left_number % right_number))
                            }
                            BinaryOperator::GreaterThan => {
                                Ok(LiteralValue::Bool(left_number > right_number))
                            }
                            BinaryOperator::LessThan => {
                                Ok(LiteralValue::Bool(left_number < right_number))
                            }
                            BinaryOperator::GreaterThanOrEqual => {
                                Ok(LiteralValue::Bool(left_number >= right_number))
                            }
                            BinaryOperator::LessThanOrEqual => {
                                Ok(LiteralValue::Bool(left_number <= right_number))
                            }
                            _ => unreachable!(),
                        }
                    }
                    BinaryOperator::Equal => Ok(LiteralValue::Bool(left_value == right_value)),
                    BinaryOperator::NotEqual => Ok(LiteralValue::Bool(left_value != right_value)),
                }
            }

            _ => Err(RuntimeError::new(
                format!("not implemented"),
                expression.span,
            )),
        }
    }

    fn execute_function(
        &mut self,
        function: &ast::SpannedStatement,
        args: &Vec<SpannedExpression>,
    ) -> Result<LiteralValue, RuntimeError> {
        for (index, arg) in args.iter().enumerate() {
            let arg_name = format!("arg{}", index);
            let arg_value = self.evaluate_expression(arg)?;
            self.scope_stack.add_statement(
                ast::Statement::VarAssignment(ast::VariableAssignment {
                    name: arg_name,
                    value: Expression::Literal(LiteralExpression {
                        value: arg_value.clone(),
                    })
                    .spanned(arg.span),
                })
                .spanned(arg.span),
            );
        }
        let Statement::Function(func) = function.node.clone() else {
            return Err(RuntimeError::new(
                format!("expected function, got {:?}", function.node),
                function.span,
            ));
        };
        match func.name.as_str() {
            "print" => {
                let arg_value = self.evaluate_expression(&args[0])?;
                print!("{}", arg_value);
            }
            "println" => {
                let arg_value = self.evaluate_expression(&args[0])?;
                println!("{}", arg_value);
            }
            "to_str" => {
                let arg_value = self.evaluate_expression(&args[0])?;
                let LiteralValue::Number(value) = arg_value else {
                    return Err(RuntimeError::new(
                        format!("not a number: {}", arg_value),
                        args[0].span,
                    ));
                };
                return Ok(LiteralValue::String(value.to_string()));
            }
            _ => {
                self.execute_body(&func.body)?;
            }
        }
        Ok(LiteralValue::Number(0))
    }

    fn execute_loop(&mut self, loop_statement: &ast::LoopStatement) -> Result<(), RuntimeError> {
        self.scope_stack.enter_scope();
        let condition_value = self.evaluate_expression(&loop_statement.condition)?;
        if condition_value == LiteralValue::Bool(false) {
            self.scope_stack.exit_scope();
            return Ok(());
        }
        self.execute_body(&loop_statement.body)?;
        self.execute_loop(loop_statement)?;
        Ok(())
    }

    fn execute_body(&mut self, body: &Vec<SpannedStatement>) -> Result<(), RuntimeError> {
        for statement in body.iter() {
            match statement.node.clone() {
                ast::Statement::Function(_) | ast::Statement::VarDecl(_) => {
                    self.scope_stack.add_statement(statement.clone())
                }
                ast::Statement::If(if_statement) => {
                    self.scope_stack.enter_scope();
                    let condition_value = self.evaluate_expression(&if_statement.condition)?;
                    if condition_value == LiteralValue::Bool(true) {
                        self.execute_body(&if_statement.then_body)?;
                    } else if let Some(else_body) = &if_statement.else_body {
                        self.execute_body(&vec![*else_body.clone()])?;
                    }
                    self.scope_stack.exit_scope();
                }
                ast::Statement::Loop(loop_statement) => self.execute_loop(&loop_statement)?,
                ast::Statement::Expr(expr) => match expr.clone() {
                    ast::Expression::FunctionCall(function_call) => {
                        self.scope_stack.enter_scope();
                        let function = {
                            let name = &function_call.callee;
                            self.scope_stack.lookup(name).cloned().ok_or_else(|| {
                                RuntimeError::new(
                                    format!("use of undefined function {}", name),
                                    statement.span,
                                )
                            })?
                        };
                        self.execute_function(&function, &function_call.args)?;
                        self.scope_stack.exit_scope();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        self.scope_stack.enter_scope();
        self.scope_builtins();
        self.execute_body(&self.program_ast.body)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    pub message: String,
    pub span: Span,
}

impl RuntimeError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuntimeError: {} (at {})", self.message, self.span)
    }
}
