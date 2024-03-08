use std::fmt::Display;
use better_term::{Color, flush_styles};
use crate::lexer::{Operator, Token, TokenType};
use crate::literal::Literal;

// AST Nodes
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // literals
    Literal(Literal),
    Ident(String),

    // block
    Block(Vec<Box<Node>>),

    // operations
    BinOp(Box<Node>, Operator, Box<Node>),
    VarDecl(String, Option<Box<Node>>),
    If(Box<Node>, Option<Box<Node>>, Option<Box<Node>>),

    Print(Box<Node>),
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
            Node::BinOp(left, op, right) => write!(f, "EQ({} {} {})", left, op, right),
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
            Node::VarDecl(ident, typ) => {
                if let Some(typ) = typ {
                    write!(f, "ASSIGN '{}' TO '{}'", ident, typ)
                } else {
                    write!(f, "ALLOCATE '{}'", ident)
                }
            }
            Node::Print(node) => write!(f, "PRINT {}", node),
            Node::EOF => write!(f, "EOF"),
        }
    }
}

// Parser
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            nodes.push(self.next());
            println!("{}Parsed: {}", Color::BrightGreen, nodes.last().unwrap());
            flush_styles()
        }
        nodes
    }

    fn next(&mut self) -> Node {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        match &token.token_type {
            TokenType::LBrace => {
                let mut nodes = Vec::new();
                while self.peek().token_type != TokenType::RBrace {
                    nodes.push(Box::new(self.next()));
                }
                self.pos += 1;
                println!("{}Block: {:?}", Color::BrightGreen, nodes);
                Node::Block(nodes)
            }

            TokenType::Let => {
                let ident = self.consume_ident();
                if self.peek().token_type == TokenType::Assign {
                    self.pos += 1;
                    let expr = self.next();
                    Node::VarDecl(ident, Some(Box::new(expr)))
                } else {
                    Node::VarDecl(ident, None)
                }
            }
            TokenType::Print => {
                let expr = self.next();
                Node::Print(Box::new(expr))
            }
            TokenType::Ident(ident) => {
                let ident = ident.clone();
                match self.peek().token_type {
                    TokenType::Assign => {
                        self.pos += 1;
                        let expr = self.next();
                        Node::VarDecl(ident, Some(Box::new(expr)))
                    }
                    _ => self.expression_stmt(Node::Ident(ident))
                }
            },

            TokenType::If => {
                // get the condition
                let cond = self.next();
                // get the then block
                let then = Some(Box::new(self.next()));
                // get the else block if it exists
                let els = if self.peek().token_type == TokenType::Else {
                    self.pos += 1; // skip the else token
                    Some(Box::new(self.next()))
                } else {
                    None
                };
                Node::If(Box::new(cond), then, els)
            }

            TokenType::Int(n) => self.expression_stmt(Node::Literal(Literal::Int(*n))),
            TokenType::Float(n) => self.expression_stmt(Node::Literal(Literal::Float(*n))),
            TokenType::String(s) => self.expression_stmt(Node::Literal(Literal::String(s.clone()))),
            TokenType::Bool(b) => self.expression_stmt(Node::Literal(Literal::Bool(*b))),

            TokenType::EOF => Node::EOF,
            _ => {
                // invalid token, dump info and exit
                println!("{}Invalid token: {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                flush_styles();
                std::process::exit(0);
            }
        }
    }

    fn consume_ident(&mut self) -> String {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        match &token.token_type {
            TokenType::Ident(ident) => ident.clone(),
            _ => {
                // invalid token, dump info and exit
                println!("{}Invalid token (ci): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                flush_styles();
                std::process::exit(0);
            }
        }
    }

    fn expression_stmt(&mut self, lhs: Node) -> Node {
        if let TokenType::Op(_) = self.peek().token_type {
            let op = self.consume_op();
            self.pos += 1;
            let rhs = self.condi();
            Node::BinOp(Box::new(lhs), op, Box::new(rhs))
        } else {
             lhs
        }
    }

    fn consume_op(&mut self) -> Operator {
        let token = &self.tokens[self.pos];
        match &token.token_type {
            TokenType::Op(op) => op.clone(),
            _ => {
                println!("{}Invalid token (co): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                flush_styles();
                std::process::exit(0);
            }
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn condi(&mut self) -> Node {
        let mut node = self.term();
        while let TokenType::Op(op) = &self.peek().token_type {
            let op = op.clone();
            match op {
                Operator::Eq | Operator::Neq | Operator::Gt | Operator::Lt | Operator::Gte | Operator::Lte => {
                    self.pos += 1;
                    let rhs = self.term();
                    node = Node::BinOp(Box::new(node), op, Box::new(rhs));
                }
                _ => break,
            }
        }
        node
    }

    fn term(&mut self) -> Node {
        let mut node = self.factor();
        while let TokenType::Op(op) = &self.peek().token_type {
            let op = op.clone();
            match op {
                Operator::Plus | Operator::Minus => {
                    self.pos += 1;
                    let rhs = self.factor();
                    node = Node::BinOp(Box::new(node), op, Box::new(rhs));
                }
                _ => break,
            }
        }
        node
    }

    fn factor(&mut self) -> Node {
        let mut node = self.power();
        while let TokenType::Op(op) = &self.peek().token_type {
            let op = op.clone();
            match op {
                Operator::Mul | Operator::Div | Operator::Mod => {
                    self.pos += 1;
                    let rhs = self.power();
                    node = Node::BinOp(Box::new(node), op, Box::new(rhs));
                }
                Operator::And | Operator::Or | Operator::Xor => {
                    self.pos += 1;
                    let rhs = self.power();
                    node = Node::BinOp(Box::new(node), op, Box::new(rhs));
                }
                Operator::LShift | Operator::RShift => {
                    self.pos += 1;
                    let rhs = self.power();
                    node = Node::BinOp(Box::new(node), op, Box::new(rhs));
                }
                _ => break,
            }
        }
        node
    }

    fn power(&mut self) -> Node {
        let mut node = self.unary();
        while let TokenType::Op(op) = &self.peek().token_type {
            let op = op.clone();
            match op {
                Operator::Pow => {
                    self.pos += 1;
                    let rhs = self.unary();
                    node = Node::BinOp(Box::new(node), op, Box::new(rhs));
                }
                _ => break,
            }
        }
        node
    }

    fn unary(&mut self) -> Node {
        if let TokenType::Op(op) = &self.peek().token_type {
            match op {
                Operator::Minus => {
                    self.pos += 1;
                    let rhs = self.unary();
                    Node::BinOp(Box::new(Node::Literal(Literal::Int(0))), Operator::Minus, Box::new(rhs))
                }
                _ => self.primary(),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Node {
        let token = &self.tokens[self.pos];
        match &token.token_type {
            TokenType::Int(n) => {
                self.pos += 1;
                Node::Literal(Literal::Int(*n))
            }
            TokenType::String(s) => {
                self.pos += 1;
                Node::Literal(Literal::String(s.clone()))
            }
            TokenType::Float(n) => {
                self.pos += 1;
                Node::Literal(Literal::Float(*n))
            }
            TokenType::Bool(b) => {
                self.pos += 1;
                Node::Literal(Literal::Bool(*b))
            }
            TokenType::Ident(ident) => {
                self.pos += 1;
                Node::Ident(ident.clone())
            }
            _ => {
                println!("{}Invalid token (p): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                flush_styles();
                std::process::exit(0);
            }
        }
    }

}