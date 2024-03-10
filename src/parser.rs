use better_term::{Color, flush_styles};
use crate::lexer::{Token, TokenType};
use crate::operator::Operator;
use crate::variable::{Variable, VariableType};
use crate::node::Node;
use crate::postfix::{ShuntedStack, ShuntedStackItem};

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
                Node::Read(VariableType::String)
            }
            TokenType::ReadInt => {
                Node::Read(VariableType::Int)
            }
            TokenType::ReadFloat => {
                Node::Read(VariableType::Float)
            }
            TokenType::ReadBool => {
                Node::Read(VariableType::Bool)
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
                let expr = self.consume_ident();
                Node::Drop(Box::new(Node::Ident(expr)))
            }
            // todo: in keyword for ranges maps and arrays
            TokenType::Ident(ident) => {
                let ident = ident.clone();
                if self.pos >= self.tokens.len() {
                    return Node::Ident(ident);
                }
                match self.peek().token_type {
                    TokenType::Assign => {
                        self.pos += 1;
                        let expr = self.next();
                        Node::VarDecl(ident, Some(Box::new(expr)))
                    }
                    // array / map access and assignment
                    TokenType::LBracket => {
                        self.pos += 1;
                        let index = self.next();
                        // check for closing bracket
                        if self.peek().token_type == TokenType::RBracket {
                            self.pos += 1;
                        } else {
                            // invalid token, dump info and exit
                            println!("{}Missing Right Bracket (']'), found: {}{:?}", Color::BrightRed, Color::Red, self.peek().token_type);
                            flush_styles();
                            std::process::exit(0);
                        }
                        if self.pos >= self.tokens.len() {
                            return Node::ArrayMapAccess(ident, Box::new(index));
                        }
                        // handle assignment or access
                        match self.peek().clone().token_type {
                            TokenType::Assign => {
                                self.pos += 1;
                                let expr = self.next();
                                Node::ArrayMapAssign(ident, Box::new(index), Box::new(expr))
                            }
                            _ => Node::ArrayMapAccess(ident, Box::new(index))
                        }
                    }
                    TokenType::In => {
                        todo!()
                    }
                    TokenType::As => {
                        self.pos += 1;
                        match self.peek().clone().token_type {
                            TokenType::VariableType(typ) => {
                                self.pos += 1;
                                Node::TypeCast(Box::new(Node::Ident(ident)), typ.clone())
                            }
                            _ => {
                                // invalid token, dump info and exit
                                println!("{}Invalid token (as): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                                flush_styles();
                                std::process::exit(0);
                            }
                        }
                    }
                    TokenType::Is => {
                        self.pos += 1;
                        match self.peek().clone().token_type {
                            TokenType::VariableType(typ) => {
                                self.pos += 1;
                                Node::TypeCheck(Box::new(Node::Ident(ident)), typ.clone())
                            }
                            _ => {
                                // invalid token, dump info and exit
                                println!("{}Invalid token (is): {}{:?}", Color::BrightRed, Color::Red, token.token_type);
                                flush_styles();
                                std::process::exit(0);
                            }
                        }
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

            TokenType::While => {
                // get the condition
                let cond = Box::new(self.next());
                // get the block
                let block = Box::new(self.next());
                Node::While(cond, block)
            }
            TokenType::Loop => {
                // get the block
                let block = Box::new(self.next());
                Node::Loop(block)
            }
            TokenType::For => {
                todo!()
            }
            TokenType::Break => {
                Node::Break
            }
            TokenType::Continue => {
                Node::Continue
            }

            // arrays and maps
            TokenType::LBracket => {
                // array = [1, 2, 3]
                match self.peek().token_type {
                    TokenType::RBracket => {
                        self.pos += 1;
                        Node::Variable(Variable::Array(Vec::new()))
                    }
                    _ => {
                        // array with elements
                        let elements = self.parse_params(TokenType::RBracket);
                        Node::Array(elements)
                    }
                }
            }
            TokenType::LBrace => {
                // map = { "a": 1, "b": 2, "c": 3 }

                todo!()
            }

            TokenType::Op(Operator::Minus) => {
                // negative sign, get the next token
                self.shunting_yard(Node::Negative)
            },

            TokenType::Op(Operator::LParen) => {
                self.shunting_yard(Node::Expression)
            }

            TokenType::Op(Operator::Not) => {
                self.shunting_yard(Node::Not)
            }

            TokenType::Int(n) => self.shunting_yard(Node::Variable(Variable::Int(*n))),
            TokenType::Float(n) => self.shunting_yard(Node::Variable(Variable::Float(*n))),
            TokenType::String(s) => self.shunting_yard(Node::Variable(Variable::String(s.clone()))),
            TokenType::Bool(b) => self.shunting_yard(Node::Variable(Variable::Bool(*b))),

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

    // parse parameters separated by commas
    // possible variants with end of ')'
    // 1. a, b, ...)
    // 2. a)
    // 3. )
    fn parse_params(&mut self, end: TokenType) -> Vec<Box<Node>> {
        let mut params = Vec::new();
        // if the next token is the end token, then there are no parameters
        if self.peek().token_type == end {
            self.pos += 1;
            return params;
        }
        loop {
            params.push(Box::new(self.next()));
            if self.peek().token_type == end {
                self.pos += 1;
                break;
            }
            if self.peek().token_type != TokenType::Comma {
                // invalid token, dump info and exit
                println!("{}Invalid parameter: {}{:?}", Color::BrightRed, Color::Red, self.peek().token_type);
                flush_styles();
                std::process::exit(0);
            }
            self.pos += 1;
        }
        params
    }

    // todo: trailing ++ or -- can cause issues
    fn shunting_yard(&mut self, lhs: Node) -> Node {
        let mut postfix = ShuntedStack::new();
        let mut op_stack = Vec::new();

        let mut last_op: Option<Operator> = None;
        let mut negative = false;
        let mut last_was_lit = false;

        match lhs {
            Node::Variable(_) | Node::Ident(_) => {
                postfix.push(ShuntedStackItem::Operand(lhs));
                last_was_lit = true;
            }
            Node::Negative => {
                negative = true;
            }
            Node::Not => {
                op_stack.push(Operator::Not);
                last_op = Some(Operator::Not);
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
                        Operator::Not => {
                            op_stack.push(op.clone());
                            last_op = Some(op.clone());
                            last_was_lit = false;
                            negative = false;
                        }
                        _ => {
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
                    if last_was_lit {
                        break;
                    }
                    postfix.push(ShuntedStackItem::Operand(Node::Variable(Variable::Int(*n))));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Float(n) => {
                    if last_was_lit {
                        break;
                    }
                    postfix.push(ShuntedStackItem::Operand(Node::Variable(Variable::Float(*n))));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Ident(ident) => {
                    if last_was_lit {
                        break;
                    }
                    postfix.push(ShuntedStackItem::Operand(Node::Ident(ident.clone())));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Bool(b) => {
                    if last_was_lit {
                        break;
                    }
                    postfix.push(ShuntedStackItem::Operand(Node::Variable(Variable::Bool(*b))));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::String(s) => {
                    if last_was_lit {
                        break;
                    }
                    postfix.push(ShuntedStackItem::Operand(Node::Variable(Variable::String(s.clone()))));
                    last_op = None;
                    last_was_lit = true;
                }
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