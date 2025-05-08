use std::{collections::HashMap, env::var, f32::consts::E, fmt::Display};

use crate::common::{
    ast::{self, Expression, LiteralExpression, LiteralValue, Span, Type},
    token::BinaryOperator,
};

/**
* Semantic analyzer
*
* Needs to check for:
* - Scoping
* - Duplicate declarations in scope
* - Use before declaration
* - Function calls match signature
* - Expression operands match type
* - Function return type matches expression
* - Nonvoid functions must return
* - *No unreachable code* (optional)
* - Literals match the context
*
*/

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

// TODO: Make this global and integrate with the parser

#[derive(Clone)]
struct Symbol {
    type_: Type,
}

#[derive(Clone)]
struct SymbolTable {
    tables: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            tables: vec![HashMap::new()],
        }
    }

    fn add_symbol(&mut self, name: String, type_: Type) {
        self.tables[0].insert(name, Symbol { type_ });
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

    fn analyze_statement(
        &mut self,
        statement: &ast::SpannedStatement,
    ) -> Result<(), SemanticError> {
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
                let expression_type = self.analyze_expression_type(&var_decl.value)?;
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
                let expression_type = self.analyze_expression_type(&var_assignment.value)?;

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
            //  TODO: function
            //  TODO: loop
            //  TODO: if
            _ => {
                //  TODO: remove once all statement types are implemented
                return Err(SemanticError {
                    message: "invalid statement".to_string(),
                    span: statement.span,
                });
            }
        }
        Ok(())
    }

    fn analyze_expression_type(
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
                        message: format!("use of undeclared variable {}", variable_ref.name),
                        span: expression.span,
                    });
                };
                Ok(symbol.clone().type_)
            }
            ast::Expression::FunctionCall(function_call) => {
                // Verify function is already declared
                let Some(symbol) = self.symbol_table.lookup(&function_call.callee) else {
                    return Err(SemanticError {
                        message: format!("use of undeclared variable {}", &function_call.callee),
                        span: expression.span,
                    });
                };
                Ok(symbol.clone().type_)
            }
            ast::Expression::Binary(binary_expression) => {
                let left_type = self.analyze_expression_type(&binary_expression.left)?;
                let right_type = self.analyze_expression_type(&binary_expression.right)?;
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

    pub fn analyze(&mut self) -> Result<(), SemanticError> {
        for statement in &self.program_ast.body {
            self.analyze_statement(statement)?;
        }
        Ok(())
    }
}
