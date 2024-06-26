use better_term::{Color, flush_styles};
use crate::debug_print;
use crate::lexer::{Token, TokenType};
use crate::operator::Operator;
use crate::variable::Variable;
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
            debug_print!("{}Parsed: {}", Color::BrightGreen, nodes.last().unwrap());
            flush_styles()
        }
        nodes
    }

    fn next(&mut self) -> Node {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        match &token.token_type {
            TokenType::LBrace => { // {
                let block = self.parse_block();
                self.pos += 1;
                // if there is a trailing operator, treat as an operand in the shunting yard
                if self.should_shunt_from_lit() {
                    self.shunting_yard(block)
                } else {
                    block
                }
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
            TokenType::Sizeof => {
                let node = self.next();
                Node::Sizeof(Box::new(node))
            }
            TokenType::Drop => {
                let ident = self.consume_ident();
                Node::Drop(ident)
            }
            TokenType::Exit => {
                Node::Exit
            }
            TokenType::Ident(ident) => {
                let node = self.parse_ident(ident.clone());
                // if there is a trailing operator, treat as an operand in the shunting yard
                if self.should_shunt_from_lit() {
                    self.shunting_yard(node)
                } else {
                    node
                }
            },

            TokenType::Fn => {
                let ident = self.consume_ident();
                // consume left paren
                if self.peek().token_type != TokenType::Op(Operator::LParen) {
                    println!("{}Missing Left Parenthesis ('('), found: {}{:?} @ {:?}", Color::BrightRed, Color::Red, self.peek().token_type, self.peek().pos);
                    flush_styles();
                    std::process::exit(0);
                }
                self.pos += 1; // consume left paren

                // get the parameter ident list
                let params = self.parse_ident_params(TokenType::Op(Operator::RParen));

                let block = Box::new(self.next());

                Node::FunctionDecl(ident, params, block)
            }

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
            TokenType::Return => {
                let expr = self.next();
                Node::Return(Some(Box::new(expr)))
            }

            // arrays and maps
            TokenType::LBracket => {
                // array = [1, 2, 3]
                let arr = self.parse_array();
                if self.should_shunt_from_lit() {
                    self.shunting_yard(arr)
                } else {
                    arr
                }
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

            // todo: this does not support `<literal> as <type>` statements
            TokenType::Int(n) => self.shunting_yard(Node::Variable(Variable::Int(*n))),
            TokenType::Float(n) => self.shunting_yard(Node::Variable(Variable::Float(*n))),
            TokenType::String(s) => self.shunting_yard(Node::Variable(Variable::String(s.clone()))),
            TokenType::Bool(b) => self.shunting_yard(Node::Variable(Variable::Bool(*b))),

            TokenType::EOF => Node::EOF,
            _ => {
                // invalid token, dump info and exit
                println!("{}Invalid token: {}{:?} @ {:?}", Color::BrightRed, Color::Red, token.token_type, token.pos);
                flush_styles();
                std::process::exit(0);
            }
        }
    }

    pub(crate) fn should_shunt_from_lit(&self) -> bool {
        if self.pos < self.tokens.len() {
            match self.peek().token_type {
                TokenType::Op(_) => true,
                TokenType::RBrace => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub(crate) fn parse_block(&mut self) -> Node {
        let mut nodes = Vec::new();
        while self.peek().token_type != TokenType::RBrace {
            nodes.push(self.next());
        }
        Node::Block(nodes)
    }

    pub(crate) fn parse_ident(&mut self, ident: String) -> Node {
        let token = &self.tokens[self.pos];
        let ident = ident.clone();
        if self.pos >= self.tokens.len() {
            return Node::Ident(ident);
        }
        match self.peek().token_type {
            TokenType::Assign => {
                self.pos += 1;
                let expr = self.next();
                Node::VarAssign(ident, Box::new(expr))
            }
            // array / map access and assignment
            TokenType::LBracket => { // [
                self.pos += 1;
                let index = self.next();
                // check for closing bracket
                if self.peek().token_type != TokenType::RBracket {
                    println!("{}Missing Right Bracket (']'), found: {}{:?} @ {:?}", Color::BrightRed, Color::Red, self.peek().token_type, self.peek().pos);
                    flush_styles();
                    std::process::exit(0);
                } else {
                    self.pos += 1;
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
            TokenType::Op(Operator::LParen) => { // (
                self.pos += 1;
                let params = self.parse_params(TokenType::Op(Operator::RParen));
                // peak future for As, Is, LBracket and In
                if self.pos >= self.tokens.len() {
                    return Node::FunctionCall(ident, params);
                }
                Node::FunctionCall(ident, params)
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
                        println!("{}Invalid token (as): {}{:?} @ {:?}", Color::BrightRed, Color::Red, token.token_type, token.pos);
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
                        println!("{}Invalid token (is): {}{:?} @ {:?}", Color::BrightRed, Color::Red, token.token_type, token.pos);
                        flush_styles();
                        std::process::exit(0);
                    }
                }
            }
            _ => Node::Ident(ident)
        }
    }

    fn parse_array(&mut self) -> Node {
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

    fn consume_ident(&mut self) -> String {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        match &token.token_type {
            TokenType::Ident(ident) => ident.clone(),
            _ => {
                // invalid token, dump info and exit
                println!("{}Invalid token (ci): {}{:?} @ {:?}", Color::BrightRed, Color::Red, token.token_type, token.pos);
                flush_styles();
                std::process::exit(0);
            }
        }
    }

    fn parse_ident_params(&mut self, end: TokenType) -> Vec<String> {
        let mut params = Vec::new();
        // if the next token is the end token, then there are no parameters
        if self.peek().token_type == end {
            self.pos += 1;
            return params;
        }
        loop {
            params.push(self.consume_ident());
            if self.peek().token_type == end {
                self.pos += 1;
                break;
            }
            if self.peek().token_type != TokenType::Comma {
                // invalid token, dump info and exit
                println!("{}Invalid parameter: {}{:?} @ {:?}", Color::BrightRed, Color::Red, self.peek().token_type, self.peek().pos);
                flush_styles();
                std::process::exit(0);
            }
            self.pos += 1;
        }
        params
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
                println!("{}Invalid parameter: {}{:?} @ {:?}", Color::BrightRed, Color::Red, self.peek().token_type, self.peek().pos);
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
            // add all others to the stack, assume parser knows best
            _ => {
                postfix.push(ShuntedStackItem::Operand(lhs));
                last_was_lit = true;
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
                                println!("{}Invalid token (aslp): {}{:?} @ {:?}", Color::BrightRed, Color::Red, token.token_type, token.pos);
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
                                // println!("{}Invalid token (asrp): {}{:?} @ {:?}", Color::BrightRed, Color::Red, token.token_type, token.pos);
                                // flush_styles();
                                // std::process::exit(0);
                                break; // this is to fix functions
                            }

                            // last_op = Some(op.clone());
                            last_was_lit = true;
                            last_op = None;
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
                TokenType::LBrace => { // {
                    // parse blocks
                    if last_was_lit {
                        break;
                    }
                    self.pos += 1;
                    let block = self.parse_block();
                    postfix.push(ShuntedStackItem::Operand(block));
                    last_op = None;
                    last_was_lit = true;
                }
                TokenType::Sizeof => {
                    if last_was_lit {
                        break;
                    }
                    self.pos += 1;
                    let next = self.next();
                    self.pos -= 1;
                    postfix.push(ShuntedStackItem::Operand(Node::Sizeof(Box::new(next))));
                    last_op = None;
                    last_was_lit = true;
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
                    // parse the ident
                    if last_was_lit {
                        break;
                    }
                    self.pos += 1;
                    let ident = self.parse_ident(ident.clone());
                    self.pos -= 1; // because peak looks at current, not future
                    postfix.push(ShuntedStackItem::Operand(ident.clone()));

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
                // has to be after ident
                TokenType::LBracket => { // [
                    // parse arrays
                    if last_was_lit {
                        break;
                    }
                    self.pos += 1;
                    let arr = self.parse_array();
                    postfix.push(ShuntedStackItem::Operand(arr));
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