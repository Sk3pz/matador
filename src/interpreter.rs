use std::collections::HashMap;
use better_term::{Color, flush_styles};
use crate::lexer::Operator;
use crate::parser::Node;

// Interpreter
pub struct Interpreter {
    env: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.eval(node);
        }
    }

    // todo: this assumes variables are only integers, no other types are supported
    fn eval(&mut self, node: Node) -> i64 {
        match node {
            Node::Literal(n) => n,
            Node::BinOp(left, op, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match op {
                    Operator::Plus => left_val + right_val,
                    Operator::Minus => left_val - right_val,
                    Operator::Mul => left_val * right_val,
                    Operator::Div => left_val / right_val,
                    Operator::Mod => left_val % right_val,
                    _ => {
                        // invalid operator, dump info and exit
                        println!("{}Unimplemented operator: {}{:?}", Color::BrightRed, Color::Red, op);
                        flush_styles();
                        std::process::exit(0);
                    },
                }
            }
            Node::Ident(ident) => {
                // todo: proper error handling
                *self.env.get(&ident).unwrap_or_else(|| panic!("Undefined variable"))
            },
            Node::Print(node) => {
                println!("{}", self.eval(*node));
                0
            }
            Node::VarDecl(ident, typ) => {
                let value = typ.map_or(0, |n| self.eval(*n));
                self.env.insert(ident, value);
                value
            }
            Node::EOF => 0,
        }
    }
}