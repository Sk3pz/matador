use std::collections::HashMap;
use better_term::{Color, flush_styles};
use crate::lexer::Operator;
use crate::literal::Literal;
use crate::parser::Node;

pub struct Interpreter {
    env: HashMap<String, Literal>,
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

    fn eval(&mut self, node: Node) -> Literal {
        match node {
            Node::Literal(n) => n,
            Node::BinOp(left, op, right) => {
                let left_val = self.eval(*left);
                let right_val = self.eval(*right);
                match op {
                    Operator::Plus => {
                        let Some(lit) = left_val.add(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Minus => {
                        let Some(lit) = left_val.sub(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Mul => {
                        let Some(lit) = left_val.mul(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Div => {
                        let Some(lit) = left_val.div(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Mod => {
                        let Some(lit) = left_val.rem(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
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
                self.env.get(&ident).unwrap_or_else(|| panic!("Undefined variable")).clone()
            },
            Node::Print(node) => {
                println!("{}", self.eval(*node));
                Literal::Int(0)
            }
            Node::VarDecl(ident, typ) => {
                let value = typ.map_or(Literal::Int(0), |n| self.eval(*n));
                self.env.insert(ident, value.clone());
                value
            }
            Node::EOF => Literal::Int(0),
        }
    }
}