use std::fmt::Display;
use crate::variable::{Variable, VariableType};
use crate::postfix::ShuntedStack;

// AST Nodes
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // variables
    Variable(Variable),
    Ident(String),

    // block
    Block(Vec<Node>),

    // operations
    ShuntedStack(ShuntedStack),
    VarDecl(String, Option<Box<Node>>),

    // array
    Array(Vec<Box<Node>>),

    // map
    Map(Vec<(Box<Node>, Box<Variable>)>),

    ArrayMapAccess(String, Box<Node>),
    ArrayMapAssign(String, Box<Node>, Box<Node>),

    TypeCast(Box<Node>, VariableType),
    TypeCheck(Box<Node>, VariableType),
    If(Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    While(Box<Node>, Box<Node>),
    Loop(Box<Node>),
    Continue,
    Break,

    // arithmetic
    Expression, // ( ... )
    Negative,

    Print(Box<Node>, bool),
    Read(VariableType),
    Drop(Box<Node>),
    EOF,
}

impl Node {
    pub fn display(nodes: &Vec<Node>) {
        for node in nodes {
            println!("{}", node);
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Variable(n) => write!(f, "LIT {}", n),
            Node::Ident(ident) => write!(f, "IDENT '{}'", ident),
            Node::Negative => write!(f, "NEG"),
            Node::Expression => write!(f, "EXPR"),
            Node::ShuntedStack(stack) => write!(f, "STACK({:?})", stack),
            Node::Block(nodes) => {
                write!(f, "BLOCK{{")?;
                for node in nodes {
                    write!(f, "{} ", node)?;
                }
                write!(f, "}}")
            }
            Node::If(cond, then, els) => {
                if let Some(then) = then {
                    if let Some(els) = els {
                        write!(f, "IF {} THEN {} ELSE {}", cond, then, els)
                    } else {
                        write!(f, "IF {} THEN {}", cond, then)
                    }
                } else {
                    if let Some(els) = els {
                        write!(f, "IF {} ELSE {}", cond, els)
                    } else {
                        write!(f, "IF {}", cond)
                    }
                }
            }
            Node::While(cond, block) => write!(f, "WHILE {} DO {}", cond, block),
            Node::Loop(block) => write!(f, "LOOP {}", block),
            Node::Continue => write!(f, "CONTINUE"),
            Node::Break => write!(f, "BREAK"),

            Node::VarDecl(ident, typ) => {
                if let Some(typ) = typ {
                    write!(f, "ASSIGN '{}' TO '{}'", ident, typ)
                } else {
                    write!(f, "ALLOCATE '{}'", ident)
                }
            }
            Node::Array(nodes) => {
                write!(f, "ARRAY[")?;
                for node in nodes {
                    write!(f, "{} ", node)?;
                }
                write!(f, "]")
            }
            Node::ArrayMapAccess(ident, index) => write!(f, "ACCESS ARRAY/MAP '{}' AT {}", ident, index),
            Node::ArrayMapAssign(ident, index, value) => write!(f, "ASSIGN ARRAY/MAP '{}' AT {} TO {}", ident, index, value),
            Node::Map(nodes) => {
                write!(f, "MAP[")?;
                for (key, value) in nodes {
                    write!(f, "{}: {} ", key, value)?;
                }
                write!(f, "]")
            }
            Node::TypeCast(node, typ) => write!(f, "CAST {} TO {}", node, typ),
            Node::TypeCheck(node, typ) => write!(f, "CHECK {} IS {}", node, typ),
            Node::Read(typ) => write!(f, "READ {}", typ),
            Node::Print(node, newline) => write!(f, "PRINT {}{}", node, if *newline { "LN" } else { "" }),
            Node::Drop(node) => write!(f, "DROP {}", node),
            Node::EOF => write!(f, "EOF"),
        }
    }
}