use std::collections::HashMap;
use better_term::{Color, flush_styles};
use crate::function::Function;
use crate::variable::Variable;

#[derive(Debug, Clone)]
struct Scope {
    variables: HashMap<String, Variable>,
    functions: HashMap<String, Function>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn push_function(&mut self, ident: String, function: Function) {
        self.functions.insert(ident, function);
    }

    fn function_exists(&self, ident: String) -> bool {
        self.functions.contains_key(&ident)
    }

    fn get_function(&self, ident: String) -> Option<&Function> {
        self.functions.get(&ident)
    }

    fn set(&mut self, ident: &str, value: Variable) {
        self.variables.insert(ident.to_string(), value);
    }

    fn get(&self, ident: &str) -> Option<&Variable> {
        self.variables.get(ident)
    }

    fn get_or_else(&self, ident: &str) -> Variable {
        self.variables.get(ident).unwrap_or_else(|| {
            println!("{}Undefined variable: {}{}", Color::BrightRed, Color::Red, ident);
            flush_styles();
            std::process::exit(0);
        }).clone()
    }

    fn remove(&mut self, ident: &str) {
        self.variables.remove(ident);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ScopeHandler {
    scopes: Vec<Scope>,
}

impl ScopeHandler {
    pub(crate) fn new() -> Self {
        ScopeHandler {
            scopes: vec![Scope::new()],
        }
    }

    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub(crate) fn set(&mut self, ident: &str, value: Variable) {
        // set a variable in one of the scopes
        for scope in self.scopes.iter_mut().rev() {
            if scope.get(ident).is_some() {
                scope.set(ident, value);
                return;
            }
        }

        // if the variable is not found in any of the scopes, set it in the current scope
        self.scopes.last_mut().unwrap().set(ident, value);
    }

    pub(crate) fn get(&self, ident: &str) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(ident) {
                return Some(value);
            }
        }
        None
    }

    pub(crate) fn get_or_else(&self, ident: &str) -> Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(ident) {
                return value.clone();
            }
        }
        println!("{}Undefined variable: {}{}", Color::BrightRed, Color::Red, ident);
        flush_styles();
        std::process::exit(0);
    }

    pub(crate) fn get_function(&self, ident: String) -> Option<&Function> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get_function(ident.clone()) {
                return Some(value);
            }
        }
        None
    }

    pub(crate) fn function_exists(&self, ident: String) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.function_exists(ident.clone()) {
                return true;
            }
        }
        false
    }

    pub(crate) fn push_function(&mut self, ident: String, function: Function) {
        self.scopes.last_mut().unwrap().push_function(ident, function);
    }

    pub(crate) fn remove(&mut self, ident: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.get(ident).is_some() {
                scope.remove(ident);
                return;
            }
        }
    }
}
