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
                    Operator::Pow => {
                        let Some(lit) = left_val.pow(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },

                    // todo: range operator

                    // conditional operators
                    Operator::Eq => {
                        let Some(lit) = left_val.eq(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Neq => {
                        let Some(lit) = left_val.neq(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Gt => {
                        let Some(lit) = left_val.gt(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Gte => {
                        let Some(lit) = left_val.gte(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Lt => {
                        let Some(lit) = left_val.lt(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Lte => {
                        let Some(lit) = left_val.lte(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },

                    // bitwise operators
                    Operator::And => {
                        let Some(lit) = left_val.and(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Or => {
                        let Some(lit) = left_val.or(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Xor => {
                        let Some(lit) = left_val.xor(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::LShift => {
                        let Some(lit) = left_val.shl(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::RShift => {
                        let Some(lit) = left_val.shr(&right_val) else {
                            println!("{}Invalid operation: {}{:?} + {:?}",
                                     Color::BrightRed, Color::Red, left_val, right_val);
                            flush_styles();
                            std::process::exit(0);
                        };
                        lit
                    },
                    Operator::Not => {
                        let Some(lit) = left_val.not() else {
                            println!("{}Invalid operation: {}{:?}", Color::BrightRed, Color::Red, left_val);
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
            Node::Block(nodes) => {
                let mut last = Literal::Int(0);
                for node in nodes {
                    last = self.eval(*node);
                }
                last
            }
            Node::Ident(ident) => {
                // todo: proper error handling
                self.env.get(&ident).unwrap_or_else(|| {
                    println!("{}Undefined variable: {}{}", Color::BrightRed, Color::Red, ident);
                    flush_styles();
                    std::process::exit(0);
                }).clone()
            },
            Node::Print(node) => {
                println!("{}", self.eval(*node));
                Literal::Int(0)
            }
            Node::Drop(node) => {
                // drop the variable if it is one
                if let Node::Ident(ident) = *node {
                    self.env.remove(&ident);
                }
                Literal::Int(0)
            }
            Node::VarDecl(ident, typ) => {
                let value = typ.map_or(Literal::Int(0), |n| self.eval(*n));
                self.env.insert(ident, value.clone());
                value
            }
            Node::If(cond, then, els) => {
                // evaluate condition
                let cond_val = self.eval(*cond);

                match cond_val {
                    Literal::Int(0) => {
                        if let Some(els) = els {
                            self.eval(*els)
                        } else {
                            Literal::Int(0)
                        }
                    },
                    Literal::Bool(b) => {
                        if b {
                            if let Some(then) = then {
                                self.eval(*then)
                            } else {
                                Literal::Int(0)
                            }
                        } else {
                            if let Some(els) = els {
                                self.eval(*els)
                            } else {
                                Literal::Int(0)
                            }
                        }
                    },
                    _ => {
                        println!("{}Invalid condition: {}{:?}", Color::BrightRed, Color::Red, cond_val);
                        flush_styles();
                        std::process::exit(0);
                    },
                }
            }
            Node::EOF => Literal::Int(0),
        }
    }
}