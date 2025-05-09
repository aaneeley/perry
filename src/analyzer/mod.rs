// TODO: Unreachable code checks
pub mod test;

use std::{collections::HashMap, fmt::Display};

use crate::common::{
    ast::{self, LiteralValue, Parameter, Span, Spannable, SpannedStatement, Type},
    token::BinaryOperator,
};

#[derive(Clone)]
struct SymbolTable {
    tables: Vec<HashMap<String, Symbol>>,
}

#[derive(Clone)]
struct Symbol {
    type_: Type,
    params: Vec<Parameter>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            tables: vec![HashMap::new()],
        }
    }

    fn add_symbol(&mut self, name: String, type_: Type) {
        self.tables[0].insert(
            name,
            Symbol {
                type_,
                params: Vec::new(),
            },
        );
    }

    fn add_function_signature(&mut self, name: String, type_: Type, params: Vec<Parameter>) {
        self.tables[0].insert(name, Symbol { type_, params });
    }

    fn enter_scope(&mut self) {
        self.tables.insert(0, HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.tables.remove(0);
    }

    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.tables.iter().find_map(|table| table.get(name))
    }
}

#[derive(Clone)]
pub struct Analyzer<'a> {
    symbol_table: SymbolTable,
    program_ast: &'a ast::Program,
}

impl<'a> Analyzer<'a> {
    pub fn new(program_ast: &'a ast::Program) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            program_ast,
        }
    }

    // Calls analyze_body on the top-level program AST
    pub fn analyze(&mut self) -> Result<(), SemanticError> {
        self.symbol_table
            .add_symbol("print".to_string(), Type::Void);
        self.symbol_table
            .add_symbol("print".to_string(), Type::Void);
        self.analyze_body(&self.program_ast.body)?;
        Ok(())
    }

    // Analyzes a body of statements. Returns a bool indicating if any statement was a return
    pub fn analyze_body(&mut self, body: &Vec<SpannedStatement>) -> Result<bool, SemanticError> {
        let mut returned = false;
        for statement in body {
            if self.analyze_statement(statement)? {
                returned = true;
            }
        }
        Ok(returned)
    }

    // Analyzes a single statement. Returns a bool indicating if the statement was a return
    fn analyze_statement(
        &mut self,
        statement: &ast::SpannedStatement,
    ) -> Result<bool, SemanticError> {
        match statement.node.clone() {
            ast::Statement::VarDecl(var_decl) => {
                let identifier = var_decl.name.clone();
                let type_ = var_decl.type_;
                if self.symbol_table.lookup(&identifier).is_some() {
                    return Err(SemanticError {
                        message: format!("duplicate declaration of {}", identifier),
                        span: statement.span,
                    });
                }
                self.symbol_table
                    .add_symbol(var_decl.name.clone(), type_.clone());
                let expression_type = self.analyze_expression(&var_decl.value)?;
                if &expression_type != &type_ {
                    return Err(SemanticError {
                        message: format!(
                            "variable {} declared with type {} but assigned with type {}",
                            identifier, type_, expression_type
                        ),
                        span: statement.span,
                    });
                }
            }
            ast::Statement::VarAssignment(var_assignment) => {
                let identifier = var_assignment.name.clone();
                let expression_type = self.analyze_expression(&var_assignment.value)?;

                // Verify variable is already declared
                let Some(symbol) = self.symbol_table.lookup(&identifier) else {
                    return Err(SemanticError {
                        message: format!("use of undeclared identifier {}", identifier),
                        span: statement.span,
                    });
                };

                // Verify variable type matches assignment type
                if symbol.type_ != expression_type {
                    return Err(SemanticError {
                        message: format!(
                            "Type mismatch: expected {}, got {}",
                            symbol.type_, expression_type
                        ),
                        span: statement.span,
                    });
                }
            }
            ast::Statement::If(if_statement) => {
                self.symbol_table.enter_scope();
                let condition_type = self.analyze_expression(&if_statement.condition)?;
                if condition_type != Type::Bool {
                    return Err(SemanticError::new(
                        format!(
                            "if statement condition must resolve to bool, got {}",
                            condition_type
                        ),
                        if_statement.condition.span,
                    ));
                }
                self.analyze_body(&if_statement.then_body)?;
                self.symbol_table.exit_scope();
                if let Some(else_body) = if_statement.else_body {
                    self.symbol_table.enter_scope();
                    self.analyze_statement(&else_body)?;
                    self.symbol_table.exit_scope();
                }
            }
            ast::Statement::Function(function) => {
                self.symbol_table.add_function_signature(
                    function.name.clone(),
                    function.type_.clone(),
                    function.params.clone(),
                );
                self.symbol_table.enter_scope();
                for param in &function.params {
                    self.symbol_table
                        .add_symbol(param.name.clone(), param.type_.clone());
                }
                self.symbol_table
                    .add_symbol("return".to_string(), function.type_.clone());
                let returned = self.analyze_body(&function.body)?;
                if function.type_ != Type::Void && !returned {
                    return Err(SemanticError::new(
                        format!("function {} must return a value", function.name),
                        statement.span,
                    ));
                }
                self.symbol_table.exit_scope();
            }
            ast::Statement::Return(return_statement) => {
                let Some(expected_return) = &self.symbol_table.lookup("return") else {
                    return Err(SemanticError::new(
                        format!("return statement outside of function"),
                        statement.span,
                    ));
                };
                let return_value_type = match &return_statement.value {
                    Some(return_value) => self.clone().analyze_expression(return_value)?,
                    None => Type::Void,
                };
                if return_value_type != expected_return.type_ {
                    return Err(SemanticError::new(
                        format!(
                            "return type mismatch: expected {}, got {}",
                            expected_return.type_, return_value_type
                        ),
                        statement.span,
                    ));
                }
                return Ok(true);
            }
            ast::Statement::Loop(loop_statement) => {
                self.symbol_table.enter_scope();
                let condition_type = self.analyze_expression(&loop_statement.condition)?;
                if condition_type != Type::Bool {
                    return Err(SemanticError::new(
                        format!(
                            "loop condition must resolve to bool, got {}",
                            condition_type
                        ),
                        loop_statement.condition.span,
                    ));
                }
                self.analyze_body(&loop_statement.body)?;
                self.symbol_table.exit_scope();
            }
            ast::Statement::Expr(expr) => {
                self.analyze_expression(&expr.spanned(statement.span))?;
            }
        }
        Ok(false)
    }

    fn analyze_expression(
        &mut self,
        expression: &ast::SpannedExpression,
    ) -> Result<Type, SemanticError> {
        match expression.node.clone() {
            ast::Expression::Literal(literal) => {
                return match literal.value {
                    LiteralValue::String(_) => Ok(Type::String),
                    LiteralValue::Number(_) => Ok(Type::Int),
                    LiteralValue::Bool(_) => Ok(Type::Bool),
                };
            }
            ast::Expression::VariableRef(variable_ref) => {
                // Verify variable is already declared
                let Some(symbol) = self.symbol_table.lookup(&variable_ref.name) else {
                    return Err(SemanticError {
                        message: format!("use of undefined variable {}", variable_ref.name),
                        span: expression.span,
                    });
                };
                Ok(symbol.clone().type_)
            }
            ast::Expression::FunctionCall(function_call) => {
                // Verify function is already declared
                let (expected_params, return_type) = {
                    let Some(symbol) = self.symbol_table.lookup(&function_call.callee) else {
                        return Err(SemanticError {
                            message: format!("use of undefined function {}", &function_call.callee),
                            span: expression.span,
                        });
                    };

                    // Clone only what’s needed and drop the borrow immediately
                    (symbol.params.clone(), symbol.type_.clone())
                };

                // Check for number of arguments
                if function_call.args.len() != expected_params.len() {
                    return Err(SemanticError {
                        message: format!(
                            "function {} expects {} arguments, got {}",
                            function_call.callee,
                            expected_params.len(),
                            function_call.args.len()
                        ),
                        span: expression.span,
                    });
                }

                // Check for type of arguments
                for (index, param) in expected_params.iter().enumerate() {
                    let arg_type = self.analyze_expression(&function_call.args[index])?;
                    if arg_type != param.type_ {
                        return Err(SemanticError {
                            message: format!(
                                "type mismatch: expected {}, got {}",
                                param.type_, arg_type
                            ),
                            span: expression.span,
                        });
                    }
                }
                Ok(return_type)
            }
            ast::Expression::Binary(binary_expression) => {
                let left_type = self.analyze_expression(&binary_expression.left)?;
                let right_type = self.analyze_expression(&binary_expression.right)?;
                if left_type != right_type {
                    return Err(SemanticError {
                        message: format!(
                            "type mismatch: expected {}, got {}",
                            left_type, right_type
                        ),
                        span: expression.span,
                    });
                }
                match binary_expression.operator {
                    BinaryOperator::LessThan
                    | BinaryOperator::LessThanOrEqual
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::GreaterThanOrEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual => Ok(Type::Bool),
                    _ => Ok(left_type),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

impl SemanticError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SemanticError: {} (at {})", self.message, self.span)
    }
}
