use std::fmt::Display;
use crate::variable::VariableType;
use crate::operator::Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // language keywords
    Let,
    Fn,
    If,
    Else,
    While,
    Loop,
    For,
    Break,
    Continue,
    In,
    As,
    Is,
    Assign,
    Op(Operator),

    // control flow
    Decimal,  // .
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]
    Colon,    // :
    Comma,    // ,

    // todo: these should become functions in the future
    // functions
    Print,
    Println,
    ReadStr,
    ReadInt,
    ReadFloat,
    ReadBool,
    Sizeof,
    Drop,

    // block outlines
    // not in operators as they are not used in the same way
    LBrace,
    RBrace,

    // identifiers
    Ident(String),

    // types
    VariableType(VariableType),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),

    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    lexeme: String,
    pub pos: (usize, usize), // line, char
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} : \"{}\"", self.token_type, self.lexeme)
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    pos: usize,
    code_pos: (usize, usize),
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().collect(),
            pos: 0,
            code_pos: (1, 0),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.chars.len() {
            let token = self.next_token();
            println!("{}: {:?}", tokens.len(), token);
            tokens.push(token);
        }
        if let Some(last) = tokens.last() {
            if last.token_type != TokenType::EOF {
                tokens.push(Token {
                    token_type: TokenType::EOF,
                    lexeme: "".to_string(),
                    pos: self.code_pos,
                });
            }
        }
        tokens
    }

    fn update_code_pos(&mut self, amt: usize) {
        self.code_pos.1 += amt;
        self.pos += amt;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let mut builder = String::new();
        // iterate through chars until a pattern is found
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            // skip comments
            if c == '/' {
                if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                    while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                        self.update_code_pos(1);
                    }
                    continue;
                } else if self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '*' {
                    self.update_code_pos(2);
                    while self.pos < self.chars.len() {
                        if self.chars[self.pos] == '*' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '/' {
                            self.update_code_pos(2);
                            break;
                        }
                        self.update_code_pos(1);
                    }
                    continue;
                }
            }
            // handle ending tokens
            if c == ')' || c == '}' || c == ']' || c == ',' || c == '[' || c == '{' {
                if !builder.is_empty() {
                    break;
                }
                builder.push(c);
                self.update_code_pos(1);
                break;
            }

            // handle leading single character tokens
            // todo: this probably wont allow x =1, and would treat =1 as an identifier
            if c == '(' {
                builder.push(c);
                self.update_code_pos(1);
                break;
            }

            if c == '!' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1] == '=' {
                if !builder.is_empty() {
                    break;
                }
                builder.push_str("!=");
                self.update_code_pos(2);
                break;
            }

            if c.is_whitespace() {
                break;
            }
            builder.push(c);
            self.update_code_pos(1);

            // handle multi-character modifiers
            if builder == "++" || builder == "--" {
                break;
            }
        }
        if builder.is_empty() {
            // handle cases where it exits the loop without adding anything to the builder
            if self.pos >= self.chars.len() {
                return Token {
                    token_type: TokenType::EOF,
                    lexeme: "".to_string(),
                    pos: self.code_pos,
                };
            }
            return self.next_token();
        }
        let token_type = match &builder[..] {
            // language keywords
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "loop" => TokenType::Loop,
            "for" => TokenType::For,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "in" => TokenType::In,
            "as" => TokenType::As,
            "is" => TokenType::Is,
            "int" => TokenType::VariableType(VariableType::Int),
            "float" => TokenType::VariableType(VariableType::Float),
            "string" => TokenType::VariableType(VariableType::String),
            "bool" => TokenType::VariableType(VariableType::Bool),
            "range" => TokenType::VariableType(VariableType::Range),
            "array" => TokenType::VariableType(VariableType::Array),
            "map" => TokenType::VariableType(VariableType::Map),

            // functions
            "readstr" | "readln" => TokenType::ReadStr,
            "readint" => TokenType::ReadInt,
            "readfloat" => TokenType::ReadFloat,
            "readbool" => TokenType::ReadBool,
            "print" => TokenType::Print,
            "println" => TokenType::Println,
            "sizeof" => TokenType::Sizeof,
            "drop" => TokenType::Drop,

            // blocks
            "{" => TokenType::LBrace,
            "}" => TokenType::RBrace,

            // control flow
            "=" => TokenType::Assign,
            "." => TokenType::Decimal,
            "[" => TokenType::LBracket,
            "]" => TokenType::RBracket,
            ":" => TokenType::Colon,
            "," => TokenType::Comma,

            // operators
            "(" => TokenType::Op(Operator::LParen),
            ")" => TokenType::Op(Operator::RParen),
            "+" => TokenType::Op(Operator::Plus),
            "-" => TokenType::Op(Operator::Minus),
            "*" => TokenType::Op(Operator::Mul),
            "/" => TokenType::Op(Operator::Div),
            "%" => TokenType::Op(Operator::Mod),
            "**" => TokenType::Op(Operator::Pow),
            ".." => TokenType::Op(Operator::Range),
            "&" => TokenType::Op(Operator::BitAnd),
            "|" => TokenType::Op(Operator::BitOr),
            "^" => TokenType::Op(Operator::Xor),
            "!" => TokenType::Op(Operator::Not),
            "<<" => TokenType::Op(Operator::LShift),
            ">>" => TokenType::Op(Operator::RShift),
            "&&" => TokenType::Op(Operator::And),
            "||" => TokenType::Op(Operator::Or),
            "++" => TokenType::Op(Operator::Inc),
            "--" => TokenType::Op(Operator::Dec),

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
                if builder.ends_with('"') {
                    builder.pop();
                    TokenType::String(builder.clone())
                } else {
                    while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
                        builder.push(self.chars[self.pos]);
                        self.update_code_pos(1);
                    }
                    self.update_code_pos(1);
                    TokenType::String(builder.clone())
                }
            },
            _ if builder.chars().all(|c| c.is_digit(10) || c == '.' || c == '-') => {
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
            pos: self.code_pos,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            if self.chars[self.pos] == '\n' {
                self.code_pos.0 += 1;
                self.code_pos.1 = 0;
            } else {
                self.code_pos.1 += 1;
            }
            self.pos += 1;
        }
    }
}