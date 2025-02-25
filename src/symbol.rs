use std::collections::HashMap;

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    data_type: DataType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, data_type: DataType) -> Self {
        Symbol {
            name,
            symbol_type,
            data_type,
        }
    }
}

pub enum SymbolType {
    Variable,
    Function,
}

pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<DataType>),
    Void,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable { scopes: Vec::new() };
        // Initialize with global scope
        table.enter_scope();
        table
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(format!("Symbol '{}' already declared in this scope", name));
            }
            scope.insert(name, symbol);
            Ok(())
        } else {
            Err("No active scope".to_string())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Look through scopes from inner to outer
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}
