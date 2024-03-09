use std::fmt::Display;
use better_term::{Color, flush_styles};
use crate::lexer::{StaticType, Token, TokenType};
use crate::operator::Operator;
use crate::literal::Literal;
use crate::postfix::{ShuntedStack, ShuntedStackItem};

// AST Nodes
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // literals
    Literal(Literal),
    Ident(String),

    // block
    Block(Vec<Node>),

    // operations
    BinOp(Box<Node>, Operator, Box<Node>),
    ShuntedStack(ShuntedStack),
    VarDecl(String, Option<Box<Node>>),
    If(Box<Node>, Option<Box<Node>>, Option<Box<Node>>),

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
            Node::Read(typ) => write!(f, "READ {}", typ),
            Node::Print(node, newline) => write!(f, "PRINT {}{}", node, if *newline { "LN" } else { "" }),
            Node::Drop(node) => write!(f, "DROP {}", node),
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
            //println!("{}Parsed: {}", Color::BrightGreen, nodes.last().unwrap());
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
                    nodes.push(self.next());
                    self.pos += 1;
                }
                self.pos += 1;
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
            TokenType::ReadStr => {
                Node::Read(StaticType::String)
            }
            TokenType::ReadInt => {
                Node::Read(StaticType::Int)
            }
            TokenType::ReadFloat => {
                Node::Read(StaticType::Float)
            }
            TokenType::ReadBool => {
                Node::Read(StaticType::Bool)
            }
            TokenType::Print => {
                let expr = self.next();
                Node::Print(Box::new(expr), false)
            }
            TokenType::Println => {
                let expr = self.next();
                Node::Print(Box::new(expr), true)
            }
            TokenType::Drop => {
                let expr = self.next();
                Node::Drop(Box::new(expr))
            }
            TokenType::Ident(ident) => {
                let ident = ident.clone();
                match self.peek().token_type {
                    TokenType::Assign => {
                        self.pos += 1;
                        let expr = self.next();
                        Node::VarDecl(ident, Some(Box::new(expr)))
                    }
                    _ => self.shunting_yard(Node::Ident(ident))
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

            TokenType::Op(Operator::Minus) => {
                // negative sign, get the next token
                self.shunting_yard(Node::Negative)
            },

            TokenType::Op(Operator::LParen) => {
                self.shunting_yard(Node::Expression)
            }

            TokenType::Int(n) => self.shunting_yard(Node::Literal(Literal::Int(*n))),
            TokenType::Float(n) => self.shunting_yard(Node::Literal(Literal::Float(*n))),
            TokenType::String(s) => self.shunting_yard(Node::Literal(Literal::String(s.clone()))),
            TokenType::Bool(b) => self.shunting_yard(Node::Literal(Literal::Bool(*b))),

            TokenType::Newline => self.next(),
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

    fn shunting_yard(&mut self, lhs: Node) -> Node {
        let mut postfix = ShuntedStack::new();
        let mut op_stack = Vec::new();

        let mut last_op: Option<Operator> = None;
        let mut negative = false;
        let mut last_was_lit = false;

        match lhs {
            Node::Literal(_) | Node::Ident(_) => {
                postfix.push(ShuntedStackItem::Operand(lhs));
            }
            Node::Negative => {
                negative = true;
            }
            Node::Expression => {
                op_stack.push(Operator::LParen);
                last_op = Some(Operator::LParen);
            }
            _ => {
                println!("{}Invalid token (as): {}{}", Color::BrightRed, Color::Red, lhs);
                flush_styles();
                std::process::exit(0);
            }
        }

        // while the next token is an operator, ident, or literal
        while self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            match &token.token_type {
                TokenType::Op(op) => {
                    match op {
                        Operator::LParen => {
                            op_stack.push(op.clone());
                            if last_was_lit {
                                // error: missing operator
                                println!("{}Invalid token (aslp): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                                flush_styles();
                                std::process::exit(0);
                            }
                            last_op = None;
                            last_was_lit = false;
                            negative = false;
                        }
                        Operator::RParen => {
                            last_was_lit = false;
                            let mut found = false;
                            while let Some(op) = op_stack.pop() {
                                if op == Operator::LParen {
                                    found = true;
                                    break;
                                }
                                postfix.push(ShuntedStackItem::Operator(op));
                            }

                            if !found {
                                // error: missing left parenthesis
                                println!("{}Invalid token (asrp): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                                flush_styles();
                                std::process::exit(0);
                            }

                            last_op = Some(op.clone());
                            negative = false;
                        }
                        Operator::Minus => {
                            if last_op.is_some() {
                                // if the last token was an operator, then this is a negative sign
                                negative = true;
                            } else {
                                // if the last token was not an operator, then this is a subtraction sign
                                op_stack.push(op.clone());
                            }
                            last_op = Some(op.clone());
                            last_was_lit = false;
                        }
                        _ => {
                            if last_op.is_some() {
                                // error: two operators in a row
                                println!("{}Invalid token (astop): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                                flush_styles();
                                std::process::exit(0);
                            }
                            last_op = Some(op.clone());
                            last_was_lit = false;
                            negative = false;
                            while let Some(op2) = op_stack.pop() {
                                if op2 == Operator::LParen {
                                    op_stack.push(op2);
                                    break;
                                }
                                if op2.precedence() < op.precedence() {
                                    op_stack.push(op2);
                                    break;
                                }
                                postfix.push(ShuntedStackItem::Operator(op2));
                            }
                            op_stack.push(op.clone());
                        }
                    }
                }
                TokenType::Int(n) => {
                    postfix.push(ShuntedStackItem::Operand(Node::Literal(Literal::Int(*n))));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Float(n) => {
                    postfix.push(ShuntedStackItem::Operand(Node::Literal(Literal::Float(*n))));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Ident(ident) => {
                    postfix.push(ShuntedStackItem::Operand(Node::Ident(ident.clone())));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Bool(b) => {
                    postfix.push(ShuntedStackItem::Operand(Node::Literal(Literal::Bool(*b))));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::String(s) => {
                    postfix.push(ShuntedStackItem::Operand(Node::Literal(Literal::String(s.clone()))));
                    last_op = None;
                    last_was_lit = true;
                }
                // exit the loop (not needed, but good to explicitly state the usage of newlines)
                TokenType::Newline => break,
                _ => break, // exit the loop
            }
            self.pos += 1;
        }

        // push the remaining operators to the postfix stack
        while let Some(op) = op_stack.pop() {
            postfix.push(ShuntedStackItem::Operator(op));
        }

        if negative {
            postfix.push(ShuntedStackItem::Operator(Operator::Minus));
        }

        Node::ShuntedStack(postfix)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

}