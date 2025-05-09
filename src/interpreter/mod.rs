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

type BuiltInFn = fn(Vec<LiteralValue>) -> Result<LiteralValue, RuntimeError>;

#[derive(Clone)]
pub struct Interpreter<'a> {
    scope_stack: ScopeStack,
    built_ins: HashMap<String, BuiltInFn>,
    program_ast: &'a ast::Program,
}

impl<'a> Interpreter<'a> {
    pub fn new(program_ast: &'a ast::Program) -> Self {
        Self {
            scope_stack: ScopeStack::new(),
            built_ins: HashMap::new(),
            program_ast,
        }
    }

    fn initialize_built_ins(&mut self) {
        self.built_ins
            .insert("print".to_string(), |args: Vec<LiteralValue>| {
                let arg_value = args[0].clone();
                print!("{}", arg_value);
                Ok(LiteralValue::Void)
            });
        self.built_ins
            .insert("println".to_string(), |args: Vec<LiteralValue>| {
                let arg_value = args[0].clone();
                println!("{}", arg_value);
                Ok(LiteralValue::Void)
            });
    }

    fn evaluate_expression(
        &mut self,
        expression: &ast::SpannedExpression,
    ) -> Result<ast::LiteralValue, RuntimeError> {
        match expression.node.clone() {
            ast::Expression::Literal(literal) => Ok(literal.value),
            ast::Expression::FunctionCall(function_call) => {
                let maybe_user_func = self.scope_stack.lookup(&function_call.callee).cloned();
                let mut evaluated_args: Vec<LiteralValue> = Vec::new();
                for arg in function_call.args {
                    let evaluated_value = self.evaluate_expression(&arg)?;
                    evaluated_args.push(evaluated_value);
                }
                self.scope_stack.enter_scope();
                if let Some(user_func) = maybe_user_func {
                    self.execute_function(&user_func, &evaluated_args)?;
                } else {
                    if let Some(built_in_fn) = self.built_ins.get(&function_call.callee) {
                        built_in_fn(evaluated_args)?;
                    } else {
                        return Err(RuntimeError::new(
                            format!("use of undefined function {}", function_call.callee),
                            expression.span,
                        ));
                    }
                }
                self.scope_stack.exit_scope();
                // TODO: returns
                Ok(LiteralValue::Void)
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
        arg_vals: &Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError> {
        let Statement::Function(func) = function.node.clone() else {
            return Err(RuntimeError::new(
                format!("expected function, got {:?}", function.node),
                function.span,
            ));
        };
        // Label arguments with their names in the function signature
        for (i, value) in arg_vals.iter().enumerate() {
            self.scope_stack.add_statement(
                ast::Statement::VarAssignment(ast::VariableAssignment {
                    name: func.params[i].name.clone(),
                    value: Expression::Literal(LiteralExpression {
                        value: value.clone(),
                    })
                    .spanned(function.span),
                })
                .spanned(function.span),
            );
        }
        self.execute_body(&func.body)?;
        // TODO: returns
        Ok(LiteralValue::Void)
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
                ast::Statement::Expr(expr) => {
                    // NOTE: Might want to filter by function calls here.
                    self.evaluate_expression(&expr.spanned(statement.span))?;
                }
                _ => panic!("invalid statement in execution body"),
            }
        }
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        self.scope_stack.enter_scope();
        self.initialize_built_ins();
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
