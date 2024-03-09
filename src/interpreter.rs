use std::collections::HashMap;
use better_term::{Color, flush_styles, read_input};
use crate::lexer::StaticType;
use crate::literal::Literal;
use crate::parser::Node;
use crate::postfix::ShuntedStackItem;

pub struct Interpreter {
    env: HashMap<String, Literal>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    fn get_var(&self, ident: &str) -> &Literal {
        self.env.get(ident).unwrap_or_else(|| {
            println!("{}Undefined variable: {}{}", Color::BrightRed, Color::Red, ident);
            flush_styles();
            std::process::exit(0);
        })
    }

    fn set_var(&mut self, ident: &str, value: Literal) {
        self.env.insert(ident.to_string(), value);
    }

    pub fn interpret(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.eval(node);
        }
    }

    fn eval(&mut self, node: Node) -> Literal {
        match node {
            Node::Literal(n) => n,
            Node::Block(nodes) => {
                let mut last = Literal::Int(0);
                for node in nodes {
                    last = self.eval(node);
                }
                last
            }
            Node::ShuntedStack(stack) => {
                //println!("{}Postfix stack: {}{:?}", Color::BrightYellow, Color::Yellow, stack);
                flush_styles();
                // interpret the stack and return the result
                let mut operand_stack: Vec<Literal> = Vec::new();
                for item in stack {
                    match item {
                        ShuntedStackItem::Operand(node) => {
                            operand_stack.push(self.eval(node));
                        },
                        ShuntedStackItem::Operator(op) => {
                            if !op.can_apply() {
                                println!("{}Invalid operator: {}{:?}", Color::BrightRed, Color::Red, op);
                                flush_styles();
                                std::process::exit(0);
                            }
                            fn pop_operand(stack: &mut Vec<Literal>) -> Literal {
                                stack.pop().unwrap_or_else(|| {
                                    println!("{}Invalid stack (no operands): {}{:?}", Color::BrightRed, Color::Red, stack);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }

                            if operand_stack.len() == 1 {
                                // handle unary operators
                                let right = pop_operand(&mut operand_stack);
                                if let Some(lit) = op.apply_unary(right.clone()) {
                                    operand_stack.push(lit);
                                } else {
                                    println!("{}Invalid operation: {}{:?}",
                                         Color::BrightRed, Color::Red, right);
                                    flush_styles();
                                    std::process::exit(0);
                                }
                                continue;
                            }

                            let right = pop_operand(&mut operand_stack);
                            let left = pop_operand(&mut operand_stack);

                            if let Some(lit) = op.apply_binary(left.clone(), right.clone()) {
                                operand_stack.push(lit);
                            } else {
                                println!("{}Invalid operation: {}{:?} + {:?}",
                                         Color::BrightRed, Color::Red, left, right);
                                flush_styles();
                                std::process::exit(0);
                            }
                        }
                    }
                }

                if operand_stack.len() != 1 {
                    println!("{}Invalid stack (too many operands): {}{:?}", Color::BrightRed, Color::Red, operand_stack);
                    flush_styles();
                    std::process::exit(0);
                }

                operand_stack.pop().unwrap()
            }
            Node::Ident(ident) => {
                self.get_var(&ident).clone()
            },
            Node::Print(node, newline) => {
                print!("{}{}", self.eval(*node), if newline { "\n" } else { "" });
                Literal::Int(0)
            }
            Node::Read(typ) => {
                let input = read_input!();
                match typ {
                    StaticType::Int => {
                        Literal::Int(input.parse().unwrap_or_else(|_| {
                            println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, input);
                            flush_styles();
                            std::process::exit(0);
                        }))
                    },
                    StaticType::Float => {
                        Literal::Float(input.parse().unwrap_or_else(|_| {
                            println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, input);
                            flush_styles();
                            std::process::exit(0);
                        }))
                    },
                    StaticType::Bool => {
                        Literal::Bool(input.parse().unwrap_or_else(|_| {
                            println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, input);
                            flush_styles();
                            std::process::exit(0);
                        }))
                    },
                    StaticType::String => {
                        Literal::String(input)
                    },
                }
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
            Node::TypeCast(ident, typ) => {
                match *ident {
                    Node::Ident(s) => {
                        // change the type of the variable
                        let value = self.get_var(&s);
                        // cast the value to the new type
                        match typ {
                            StaticType::Int => {
                                // cast the value to an int
                                value.to_int()
                            }
                            StaticType::Float => {
                                // cast the value to a float
                                value.to_float()
                            }
                            StaticType::String => {
                                // cast the value to a string
                                value.to_string()
                            }
                            StaticType::Bool => {
                                value.to_bool()
                            }
                        }
                    }
                    _ => {
                        println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                        flush_styles();
                        std::process::exit(0);
                    }
                }
            }
            Node::TypeCheck(ident, typ) => {
                match *ident {
                    Node::Ident(s) => {
                        // check the type of the variable
                        let value = self.get_var(&s);
                        // check if the value is of the correct type
                        match *value {
                            Literal::Int(_) => {
                                if typ == StaticType::Int {
                                    Literal::Bool(true)
                                } else {
                                    Literal::Bool(false)
                                }
                            }
                            Literal::Float(_) => {
                                if typ == StaticType::Float {
                                    Literal::Bool(true)
                                } else {
                                    Literal::Bool(false)
                                }
                            }
                            Literal::String(_) => {
                                if typ == StaticType::String {
                                    Literal::Bool(true)
                                } else {
                                    Literal::Bool(false)
                                }
                            }
                            Literal::Bool(_) => {
                                if typ == StaticType::Bool {
                                    Literal::Bool(true)
                                } else {
                                    Literal::Bool(false)
                                }
                            }
                        }
                    }
                    _ => {
                        println!("{}Invalid type check: {}{:?}", Color::BrightRed, Color::Red, ident);
                        flush_styles();
                        std::process::exit(0);
                    }
                }
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
            _ => {
                println!("{}Unexpected node: {}{:?}", Color::BrightRed, Color::Red, node);
                flush_styles();
                std::process::exit(0);
            }
        }
    }
}