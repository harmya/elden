use std::collections::HashMap;

use crate::{
    program::Program,
    symbol::{DataType, Symbol, SymbolTable, SymbolType},
    token::Token,
};

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    current_function: Option<String>,
    errors: Vec<String>,
}

// Inferred type for expressions
pub enum InferredType {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<InferredType>),
    Unknown,
    Void,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            current_function: None,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<String>> {
        // First pass: register all functions in the symbol table
        for function in &program.functions {
            let func_name = match &function.name {
                Token::Identifier(name) => name.clone(),
                _ => {
                    self.errors
                        .push("Function name must be an identifier".to_string());
                    continue;
                }
            };

            let symbol = Symbol::new(func_name.clone(), SymbolType::Function, DataType::Void);

            if let Err(err) = self.symbol_table.declare(func_name.clone(), symbol) {
                self.errors.push(err);
            }
        }

        for function in &program.functions {
            let func_name = match &function.name {
                Token::Identifier(name) => name.clone(),
                _ => continue,
            };

            self.current_function = Some(func_name);
            self.symbol_table.enter_scope();

            // Register parameters
            for param in &function.params {
                match param {
                    Token::Identifier(name) => {
                        let symbol =
                            Symbol::new(name.clone(), SymbolType::Variable, DataType::Integer);

                        if let Err(err) = self.symbol_table.declare(name.clone(), symbol) {
                            self.errors.push(err);
                        }
                    }
                    _ => self
                        .errors
                        .push("Function parameter must be an identifier".to_string()),
                }
            }

            for statement in &function.body {
                todo!()
            }

            self.symbol_table.exit_scope();
            self.current_function = None;
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
}
