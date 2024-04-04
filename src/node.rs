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
    VarAssign(String, Box<Node>),

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
    fn to_display_string(&self, depth: &u32) -> String {
        let spaces = "  ".repeat(*depth as usize);
        format!("{}{}", spaces, match self {
            Node::Variable(v) => { format!("Variable: {}", v) }
            Node::Ident(ident) => { format!("Ident: {}", ident) }
            Node::Negative => { "Negative".to_string() }
            Node::Not => { "Not".to_string() }
            Node::Expression => { "Expression".to_string() }
            Node::ShuntedStack(stack) => { format!("ShuntedStack: {:?}", stack) }
            Node::Block(nodes) => {
                let mut s = "Block:\n".to_string();
                for node in nodes {
                    s.push_str(&node.to_display_string(&(*depth + 1)));
                    s.push_str("\n");
                }
                s
            }
            Node::FunctionDecl(ident, args, block) => {
                let mut s = format!("FunctionDecl: {}(", ident);
                for arg in args {
                    s.push_str(&arg.to_string());
                    s.push_str(",");
                }
                s.push_str(")\n");
                s.push_str(&block.to_display_string(&(*depth + 1)));
                s
            }
            Node::FunctionCall(ident, args) => {
                let mut s = format!("FunctionCall: {}\n", ident);
                for arg in args {
                    s.push_str(&arg.to_string());
                    s.push_str("\n");
                }
                s
            }
            Node::If(cond, then, els) => {
                let mut s = format!("If: {}\n", cond);
                if let Some(then) = then {
                    s.push_str(&then.to_display_string(&(*depth + 1)));
                    s.push_str("\n");
                }
                if let Some(els) = els {
                    s.push_str(&els.to_display_string(&(*depth + 1)));
                    s.push_str("\n");
                }
                s
            }
            Node::While(cond, block) => {
                let mut s = format!("While: {}\n", cond);
                s.push_str(&block.to_display_string(&(*depth + 1)));
                s
            }
            Node::Loop(block) => { format!("Loop: {}", block) }
            Node::Continue => { "Continue".to_string() }
            Node::Break => { "Break".to_string() }
            Node::Return(v) => {
                if let Some(v) = v {
                    format!("Return: {}", v)
                } else {
                    "Return".to_string()
                }
            }
            Node::VarDecl(ident, typ) => {
                if let Some(typ) = typ {
                    format!("VarDecl: {} to {}", ident, typ)
                } else {
                    format!("VarDecl: {}", ident)
                }
            }
            Node::VarAssign(ident, value) => { format!("VarAssign: {} to {}", ident, value) }
            Node::Array(nodes) => {
                let mut s = "Array:\n".to_string();
                for node in nodes {
                    s.push_str(&node.to_display_string(&(*depth + 1)));
                    s.push_str("\n");
                }
                s
            }
            Node::ArrayMapAccess(ident, index) => { format!("ArrayMapAccess: {} at {}", ident, index) }
            Node::ArrayMapAssign(ident, index, value) => { format!("ArrayMapAssign: {} at {} to {}", ident, index, value) }
            Node::Map(nodes) => {
                let mut s = "Map:\n".to_string();
                for (key, value) in nodes {
                    s.push_str(&key.to_display_string(&(*depth + 1)));
                    s.push_str(&value.to_string());
                    s.push_str("\n");
                }
                s
            }
            Node::TypeCast(node, typ) => { format!("TypeCast: {} to {}", node, typ) }
            Node::TypeCheck(node, typ) => { format!("TypeCheck: {} is {}", node, typ) }
            Node::Sizeof(ident) => { format!("Sizeof: {}", ident) }
            Node::Drop(node) => { format!("Drop: {}", node) }
            Node::Exit => { "Exit".to_string() }
            Node::EOF => { "EOF".to_string() }
            _ => { "???".to_string() }
        })
    }

    pub fn display(&self, depth: u32) {
        // print depth spaces
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", self.to_display_string(&depth));
    }

    pub fn prgm_display(nodes: &Vec<Node>) {
        let depth = 0;
        for node in nodes {
            node.display(depth);
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
                writeln!(f, "BLOCK {{")?;
                for node in nodes {
                    writeln!(f, "{} ", node)?;
                }
                write!(f, "}}")
            }
            Node::FunctionDecl(ident, args, block) => {
                writeln!(f, "FUNCTION '{}'(", ident)?;
                for arg in args {
                    writeln!(f, "{} ", arg)?;
                }
                write!(f, ") {}", block)
            }
            Node::FunctionCall(ident, args) => {
                write!(f, "CALL '{}'(", ident)?;
                for arg in args {
                    writeln!(f, "{} ", arg)?;
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
            Node::VarAssign(ident, value) => write!(f, "ASSIGN '{}' TO {}", ident, value),
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