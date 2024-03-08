// Token types
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Let,
    Print,
    Ident(String),
    Int,
    Assign,
    Number(i64),
    Op(Operator),
    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Plus,  // +
    Minus, // -
    Mul,   // *
    Div,   // /
    Mod,   // %
    Eq,    // ==
    Neq,   // !=
    Gt,    // >
    Lt,    // <
    Gte,   // >=
    Lte,   // <=
    LParen, // (
    RParen, // )
}

// Token struct
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    lexeme: String,
}

// Lexer
pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.chars.len() {
            let token = self.next_token();
            tokens.push(token);
        }
        tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
        });
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut builder = String::new();
        // iterate through chars until a pattern is found
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c.is_whitespace() {
                break;
            }
            builder.push(c);
            self.pos += 1;
        }
        if builder.is_empty() {
            return Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
            };
        }
        let token_type = match &builder[..] {
            "let" => TokenType::Let,
            "print" => TokenType::Print,
            "int" => TokenType::Int,
            "=" => TokenType::Assign,
            "+" => TokenType::Op(Operator::Plus),
            "-" => TokenType::Op(Operator::Minus),
            "*" => TokenType::Op(Operator::Mul),
            "/" => TokenType::Op(Operator::Div),
            "%" => TokenType::Op(Operator::Mod),
            "==" => TokenType::Op(Operator::Eq),
            "!=" => TokenType::Op(Operator::Neq),
            ">" => TokenType::Op(Operator::Gt),
            "<" => TokenType::Op(Operator::Lt),
            ">=" => TokenType::Op(Operator::Gte),
            "<=" => TokenType::Op(Operator::Lte),
            _ if builder.chars().all(|c| c.is_digit(10)) => TokenType::Number(builder.parse().unwrap()),
            _ => TokenType::Ident(builder.clone()),
        };

        Token {
            token_type,
            lexeme: builder,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}