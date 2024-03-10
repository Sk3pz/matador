use std::collections::HashMap;
use better_term::{Color, flush_styles};
use crate::literal::Literal;

#[derive(Debug, Clone, PartialEq)]
struct Scope {
    variables: HashMap<String, Literal>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }

    fn set(&mut self, ident: &str, value: Literal) {
        self.variables.insert(ident.to_string(), value);
    }

    fn get(&self, ident: &str) -> Option<&Literal> {
        self.variables.get(ident)
    }

    fn get_or_else(&self, ident: &str) -> Literal {
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

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeHandler {
    scopes: Vec<Scope>,
}

impl ScopeHandler {
    pub fn new() -> Self {
        ScopeHandler {
            scopes: vec![Scope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn set(&mut self, ident: &str, value: Literal) {
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

    pub fn get(&self, ident: &str) -> Option<&Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(ident) {
                return Some(value);
            }
        }
        None
    }

    pub fn get_or_else(&self, ident: &str) -> Literal {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(ident) {
                return value.clone();
            }
        }
        println!("{}Undefined variable: {}{}", Color::BrightRed, Color::Red, ident);
        flush_styles();
        std::process::exit(0);
    }

    pub fn remove(&mut self, ident: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.get(ident).is_some() {
                scope.remove(ident);
                return;
            }
        }
    }
}
