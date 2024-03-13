use better_term::{Color, flush_styles};
use crate::function::{Function, NativeFunction};
use crate::variable::{Variable, VariableType};
use crate::node::Node;
use crate::operator::Operator;
use crate::postfix::ShuntedStackItem;
use crate::scope::ScopeHandler;

#[derive(Debug, PartialEq, Clone)]
enum InterFlag {
    Break,
    Continue,
    Return(Option<Box<Node>>),
}

pub struct Interpreter {
    env: ScopeHandler,
    flag: Option<InterFlag>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: ScopeHandler::new(),
            flag: None,
        }
    }

    pub fn interpret(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.eval(node);
        }
    }

    pub fn register_native_function<S: Into<String>>(&mut self, name: S, f: NativeFunction) {
        self.env.push_function(name.into(), Function::Native(f));
    }

    fn eval(&mut self, node: Node) -> Variable {
        if self.flag.is_some() {
            return Variable::Int(0);
        }
        match node.clone() {
            Node::Variable(n) => n,
            Node::Block(nodes) => {
                let mut last = Variable::Int(0);
                // create a new scope
                self.env.push_scope();
                for node in nodes {
                    last = self.eval(node);
                    if let Some(f) = &self.flag {
                        match f {
                            InterFlag::Break => break,
                            InterFlag::Continue => continue,
                            InterFlag::Return(_) => break
                        }
                    }
                }
                // remove the scope
                self.env.pop_scope();
                last
            }
            Node::ShuntedStack(stack) => {
                flush_styles();
                // interpret the stack and return the result
                let mut operand_stack: Vec<Variable> = Vec::new();
                for item in stack {
                    match item {
                        ShuntedStackItem::Operand(node) => {
                            operand_stack.push(self.eval(node));
                        },
                        ShuntedStackItem::Operator(Operator::Minus) => {
                            if operand_stack.len() == 1 {
                                let right = operand_stack.pop().unwrap();
                                operand_stack.push(right.neg().unwrap_or_else(|| {
                                    println!("{}Invalid operation (syom): {}{:?}", Color::BrightRed, Color::Red, right);
                                    flush_styles();
                                    std::process::exit(0);
                                }));
                            } else {
                                let right = operand_stack.pop().unwrap();
                                let left = operand_stack.pop().unwrap();
                                if let Some(lit) = Operator::Minus.apply_binary(left.clone(), right.clone()) {
                                    operand_stack.push(lit);
                                } else {
                                    println!("{}Invalid operation (syobm): {}{:?} - {:?}",
                                         Color::BrightRed, Color::Red, left, right);
                                    flush_styles();
                                    std::process::exit(0);
                                }
                            }
                        },
                        ShuntedStackItem::Operator(op) => {
                            if !op.can_apply() {
                                println!("{}Invalid operator: {}{:?}", Color::BrightRed, Color::Red, op);
                                flush_styles();
                                std::process::exit(0);
                            }
                            fn pop_operand(stack: &mut Vec<Variable>) -> Variable {
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
                                    println!("{}Invalid operation (syops): {}{:?}",
                                         Color::BrightRed, Color::Red, right);
                                    flush_styles();
                                    std::process::exit(0);
                                }
                                continue;
                            }

                            let right = pop_operand(&mut operand_stack);

                            // unary check
                            if op.is_unary() {
                                if let Some(lit) = op.apply_unary(right.clone()) {
                                    operand_stack.push(lit);
                                } else {
                                    println!("{}Invalid operation (syau): {}{:?} {} ; {:?}",
                                         Color::BrightRed, Color::Red, op, right, node);
                                    flush_styles();
                                    std::process::exit(0);
                                }
                                continue;
                            }

                            let left = pop_operand(&mut operand_stack);

                            if let Some(lit) = op.apply_binary(left.clone(), right.clone()) {
                                operand_stack.push(lit);
                            } else {
                                println!("{}Invalid operation (syab): {}{:?} {} {:?} ; {:?}",
                                         Color::BrightRed, Color::Red, left, op, right, node);
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
                self.env.get_or_else(&ident).clone()
            },
            Node::Sizeof(node) => {
                let value = self.eval(*node.clone());
                value.sizeof().unwrap_or_else(|| {
                    println!("{}Invalid sizeof: {}{:?}", Color::BrightRed, Color::Red, node);
                    flush_styles();
                    std::process::exit(0);
                })
            }
            Node::Drop(ident) => {
                // drop the variable if it is one
                self.env.remove(&ident);
                Variable::Int(0)
            }
            Node::Exit => {
                flush_styles();
                std::process::exit(0);
            }
            Node::VarDecl(ident, typ) => {
                let value = typ.map_or(Variable::Int(0), |n| self.eval(*n));
                self.env.set(&ident, value.clone());
                value
            }
            Node::Array(nodes) => {
                let mut array = Vec::new();
                for node in nodes {
                    array.push(Box::new(self.eval(*node)))
                }
                Variable::Array(array)
            }
            Node::ArrayMapAccess(ident, index) => {
                // get the array if it exists from the environment
                let array = self.env.get_or_else(&ident).to_array().unwrap_or_else(|| {
                    println!("{}Invalid array access: {}{:?}", Color::BrightRed, Color::Red, ident);
                    flush_styles();
                    std::process::exit(0);
                });

                // get the index
                let i = self.eval(*index);

                // get the value from the array
                array.access(i.clone()).unwrap_or_else(|| {
                    println!("{}Invalid array index: {}{:?}", Color::BrightRed, Color::Red, i);
                    flush_styles();
                    std::process::exit(0);
                })
            }
            Node::ArrayMapAssign(ident, index, value) => {
                // get the array if it exists from the environment
                let mut array = self.env.get_or_else(&ident).to_array().unwrap_or_else(|| {
                    println!("{}Invalid array assign: {}{:?}", Color::BrightRed, Color::Red, ident);
                    flush_styles();
                    std::process::exit(0);
                });

                // get the index
                let i = self.eval(*index);

                // get the value from the array
                let v = self.eval(*value);

                // set the value in the array
                array.assign(i.clone(), v).unwrap_or_else(|| {
                    println!("{}Invalid array index: {}{:?}", Color::BrightRed, Color::Red, i);
                    flush_styles();
                    std::process::exit(0);
                });

                // update the array in the environment
                self.env.set(&ident, array);
                Variable::Int(0)
            }
            Node::TypeCast(ident, typ) => {
                match *ident {
                    Node::Ident(ref s) => {
                        // change the type of the variable
                        let value = self.env.get_or_else(&s);
                        // cast the value to the new type
                        match typ {
                            VariableType::Int => {
                                // cast the value to an int
                                value.to_int().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }
                            VariableType::Float => {
                                // cast the value to a float
                                value.to_float().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }
                            VariableType::String => {
                                // cast the value to a string
                                value.to_string().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }
                            VariableType::Bool => {
                                value.to_bool().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }
                            VariableType::Array => {
                                value.to_array().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }
                            VariableType::Map => {
                                value.to_map().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
                            }
                            VariableType::Range => {
                                value.to_range().unwrap_or_else(|| {
                                    println!("{}Invalid type cast: {}{:?}", Color::BrightRed, Color::Red, ident);
                                    flush_styles();
                                    std::process::exit(0);
                                })
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
                        let value = self.env.get_or_else(&s);
                        // check if the value is of the correct type
                        match value {
                            Variable::Int(_) => {
                                if typ == VariableType::Int {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
                                }
                            }
                            Variable::Float(_) => {
                                if typ == VariableType::Float {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
                                }
                            }
                            Variable::String(_) => {
                                if typ == VariableType::String {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
                                }
                            }
                            Variable::Bool(_) => {
                                if typ == VariableType::Bool {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
                                }
                            }
                            Variable::Range(_, _) => {
                                if typ == VariableType::Range {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
                                }
                            }
                            Variable::Array(_) => {
                                if typ == VariableType::Array {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
                                }
                            }
                            Variable::Map(_) => {
                                if typ == VariableType::Map {
                                    Variable::Bool(true)
                                } else {
                                    Variable::Bool(false)
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
            Node::FunctionDecl(ident, args, body) => {
                self.env.push_function(ident, Function::Local(args, *body));
                Variable::Int(0)
            }
            Node::FunctionCall(ident, args) => {
                let func = self.env.get_function(ident.clone()).unwrap_or_else(|| {
                    println!("{}Invalid function call: {}{:?}", Color::BrightRed, Color::Red, ident);
                    flush_styles();
                    std::process::exit(0);
                }).clone();

                match func {
                    Function::Native(f) => {
                        let mut vars = Vec::new();
                        for arg in args {
                            vars.push(self.eval(*arg));
                        }
                        f(vars)
                    }
                    Function::Local(params, body) => {
                        // create a new scope for the function
                        self.env.push_scope();
                        // ensure the number of arguments matches the number of parameters
                        if params.len() != args.len() {
                            println!("{}Invalid number of arguments: {}{:?}", Color::BrightRed, Color::Red, params);
                            flush_styles();
                            std::process::exit(0);
                        }
                        // set the parameters
                        for (param, arg) in params.iter().zip(args.iter()) {
                            let p = self.eval(*arg.clone());
                            self.env.set(param, p);
                        }
                        // run the body
                        let mut result = self.eval(body.clone());
                        if let Some(f) = self.flag.clone() {
                            if let InterFlag::Return(v) = f {
                                self.flag = None;
                                if let Some(v) = v {
                                    result = self.eval(*v);
                                }
                            }
                        }
                        // remove the scope
                        self.env.pop_scope();
                        result
                    }
                }
            }
            Node::If(cond, then, els) => {
                // evaluate condition
                let cond_val = self.eval(*cond);

                match cond_val {
                    Variable::Int(0) => {
                        if let Some(els) = els {
                            self.eval(*els)
                        } else {
                            Variable::Int(0)
                        }
                    },
                    Variable::Bool(b) => {
                        if b {
                            if let Some(then) = then {
                                self.eval(*then)
                            } else {
                                Variable::Int(0)
                            }
                        } else {
                            if let Some(els) = els {
                                self.eval(*els)
                            } else {
                                Variable::Int(0)
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
            Node::While(cond, body) => {
                let last = Variable::Int(0);
                loop {
                    if let Some(f) = &self.flag {
                        match f {
                            InterFlag::Break => {
                                self.flag = None;
                                break;
                            },
                            InterFlag::Continue => {
                                self.flag = None;
                                continue;
                            },
                            InterFlag::Return(_) => break
                        }
                    }
                    // evaluate condition
                    let condition = self.eval(*cond.clone()).to_bool().unwrap_or_else(|| {
                        println!("{}Invalid condition: {}{:?}", Color::BrightRed, Color::Red, cond);
                        flush_styles();
                        std::process::exit(0);
                    });
                    let Variable::Bool(c) = condition else { break; };
                    if !c { break; }

                    // run the body
                    self.eval(*body.clone());
                }

                last
            }
            Node::Loop(body) => {
                let last = Variable::Int(0);
                'top: loop {
                    match self.flag {
                        Some(InterFlag::Break) => {
                            self.flag = None;
                            break 'top;
                        },
                        Some(InterFlag::Continue) => {
                            self.flag = None;
                            continue 'top;
                        },
                        Some(InterFlag::Return(_)) => break,
                        _ => {}
                    }
                    // run the body
                    self.eval(*body.clone());
                }
                last
            }
            Node::Break => {
                self.flag = Some(InterFlag::Break);
                Variable::Int(0)
            }
            Node::Continue => {
                self.flag = Some(InterFlag::Continue);
                Variable::Int(0)
            }
            Node::Return(v) => {
                self.flag = Some(InterFlag::Return(v));
                Variable::Int(0)
            }

            Node::EOF => Variable::Int(0),
            _ => {
                println!("{}Unexpected node: {}{:?}", Color::BrightRed, Color::Red, node);
                flush_styles();
                std::process::exit(0);
            }
        }
    }
}