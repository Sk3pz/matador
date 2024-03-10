use std::fmt::Display;
use crate::literal::{Literal, StaticType};
use crate::postfix::ShuntedStack;

// AST Nodes
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // literals
    Literal(Literal),
    Ident(String),

    // block
    Block(Vec<Node>),

    // operations
    ShuntedStack(ShuntedStack),
    VarDecl(String, Option<Box<Node>>),
    TypeCast(Box<Node>, StaticType),
    TypeCheck(Box<Node>, StaticType),
    If(Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    While(Box<Node>, Box<Node>),

    // arithmetic
    Expression, // ( ... )
    Negative,

    Print(Box<Node>, bool),
    Read(StaticType),
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
            Node::Literal(n) => write!(f, "LIT {}", n),
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
            Node::VarDecl(ident, typ) => {
                if let Some(typ) = typ {
                    write!(f, "ASSIGN '{}' TO '{}'", ident, typ)
                } else {
                    write!(f, "ALLOCATE '{}'", ident)
                }
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