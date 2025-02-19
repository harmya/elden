use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    // In a more complete compiler, you might include type, scope level, etc.
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolInfo>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, info: SymbolInfo) -> Result<(), String> {
        if self.symbols.contains_key(name) {
            return Err(format!("Duplicate declaration of symbol: {}", name));
        }
        self.symbols.insert(name.to_string(), info);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }
}
