use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // language keywords
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    In,
    Print,
    Assign,
    Op(Operator),

    // block outlines
    // not in operators as they are not used in the same way
    LBrace,
    RBrace,

    // identifiers
    Ident(String),

    // types
    StaticType(StaticType),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),

    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StaticType {
    Int,
    Float,
    String,
    Bool,
}

impl Display for StaticType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let typ = match self {
            StaticType::Int => "int",
            StaticType::Float => "float",
            StaticType::String => "string",
            StaticType::Bool => "bool",
        };
        write!(f, "{}", typ)
    }

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    // arithmetic
    Plus,   // +
    Minus,  // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    Pow,    // **

    // misc
    Dec,    // .
    Range,  // ..

    // control flow
    LParen, // (
    RParen, // )
    LBracket, // [
    RBracket, // ]

    // bitwise
    And,    // &
    Or,     // |
    Xor,    // ^
    Not,    // !
    LShift, // <<
    RShift, // >>

    // conditionals
    Eq,     // ==
    Neq,    // !=
    Gt,     // >
    Lt,     // <
    Gte,    // >=
    Lte,    // <=
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Pow => "**",
            Operator::Dec => ".",
            Operator::Range => "..",
            Operator::LParen => "(",
            Operator::RParen => ")",
            Operator::LBracket => "[",
            Operator::RBracket => "]",
            Operator::And => "&",
            Operator::Or => "|",
            Operator::Xor => "^",
            Operator::Not => "~",
            Operator::LShift => "<<",
            Operator::RShift => ">>",
            Operator::Eq => "==",
            Operator::Neq => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
        };
        write!(f, "{}", op)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    lexeme: String,
}

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
            println!("{:?}", token);
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
            // skip comments
            if c == '/' {
                if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                    while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                        self.pos += 1;
                    }
                    continue;
                } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '*' {
                    self.pos += 2;
                    while self.pos < self.chars.len() {
                        if self.chars[self.pos] == '*' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                    continue;
                }
            }
            builder.push(c);
            self.pos += 1;
        }
        if builder.is_empty() {
            // handle cases where it exits the loop without adding anything to the builder
            return self.next_token();
        }
        let token_type = match &builder[..] {
            // language keywords
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "in" => TokenType::In,
            "print" => TokenType::Print,

            // static type definitions
            "int" => TokenType::StaticType(StaticType::Int),
            "float" => TokenType::StaticType(StaticType::Float),
            "string" => TokenType::StaticType(StaticType::String),
            "bool" => TokenType::StaticType(StaticType::Int),

            // operators
            "=" => TokenType::Assign,
            "+" => TokenType::Op(Operator::Plus),
            "-" => TokenType::Op(Operator::Minus),
            "*" => TokenType::Op(Operator::Mul),
            "/" => TokenType::Op(Operator::Div),
            "%" => TokenType::Op(Operator::Mod),
            "**" => TokenType::Op(Operator::Pow),
            "." => TokenType::Op(Operator::Dec),
            ".." => TokenType::Op(Operator::Range),
            "(" => TokenType::Op(Operator::LParen),
            ")" => TokenType::Op(Operator::RParen),
            "[" => TokenType::Op(Operator::LBracket),
            "]" => TokenType::Op(Operator::RBracket),
            "{" => TokenType::LBrace,
            "}" => TokenType::RBrace,
            "&" => TokenType::Op(Operator::And),
            "|" => TokenType::Op(Operator::Or),
            "^" => TokenType::Op(Operator::Xor),
            "~" => TokenType::Op(Operator::Not),
            "<<" => TokenType::Op(Operator::LShift),
            ">>" => TokenType::Op(Operator::RShift),

            // conditionals
            "==" => TokenType::Op(Operator::Eq),
            "!=" => TokenType::Op(Operator::Neq),
            ">" => TokenType::Op(Operator::Gt),
            "<" => TokenType::Op(Operator::Lt),
            ">=" => TokenType::Op(Operator::Gte),
            "<=" => TokenType::Op(Operator::Lte),

            // literals
            "true" | "false" => {
                TokenType::Bool(builder == "true")
            }
            _ if builder.starts_with('"') => { // string literals
                builder = String::from(&builder[1..]);
                while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
                    builder.push(self.chars[self.pos]);
                    self.pos += 1;
                }
                self.pos += 1;
                TokenType::String(builder.clone())
            },
            _ if builder.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') => {
                // todo: math does not allow for parentheses right now
                if builder.contains('.') {
                    TokenType::Float(builder.parse().unwrap())
                } else {
                    TokenType::Int(builder.parse().unwrap())
                }
            },

            // identifiers
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