use std::fmt::Display;
use crate::variable::{Variable, VariableType};
use crate::postfix::ShuntedStack;

// AST Nodes
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Node {
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

    FunctionDecl(String, Vec<String>, Box<Node>),
    FunctionCall(String, Vec<Box<Node>>),
    If(Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    While(Box<Node>, Box<Node>),
    Loop(Box<Node>),
    Continue,
    Break,
    Return(Option<Box<Node>>),

    // arithmetic
    Expression, // ( ... )
    Negative,
    Not,

    Sizeof(Box<Node>),
    Drop(String),
    Exit,
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
            Node::Not => write!(f, "NOT"),
            Node::Expression => write!(f, "EXPR"),
            Node::ShuntedStack(stack) => write!(f, "STACK({:?})", stack),
            Node::Block(nodes) => {
                write!(f, "BLOCK{{")?;
                for node in nodes {
                    write!(f, "{} ", node)?;
                }
                write!(f, "}}")
            }
            Node::FunctionDecl(ident, args, block) => {
                write!(f, "FUNCTION '{}'(", ident)?;
                for arg in args {
                    write!(f, "{} ", arg)?;
                }
                write!(f, ") {}", block)
            }
            Node::FunctionCall(ident, args) => {
                write!(f, "CALL '{}'(", ident)?;
                for arg in args {
                    write!(f, "{} ", arg)?;
                }
                write!(f, ")")
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
            Node::Return(v) => {
                if let Some(v) = v {
                    write!(f, "RETURN {}", v)
                } else {
                    write!(f, "RETURN")
                }
            }

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
            Node::Sizeof(ident) => write!(f, "SIZEOF {}", ident),
            Node::Drop(node) => write!(f, "DROP {}", node),
            Node::Exit => write!(f, "EXIT"),
            Node::EOF => write!(f, "EOF"),
        }
    }
}