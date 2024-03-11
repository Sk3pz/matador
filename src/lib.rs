// this is for embedding matador in other programs
use crate::lexer::Lexer;
use crate::parser::Parser;

mod parser;
mod node;
mod interpreter;
mod lexer;
mod variable;
mod postfix;
mod operator;
mod scope;
mod debug;
mod function;
mod matador_std;

pub const DEBUG_OUTPUT: bool = false;

pub struct Matador {
    code: String,
}

impl Matador {
    pub fn new(code: String) -> Self {
        Self {
            code
        }
    }

    pub fn execute(&self) {
        // lexer
        let mut lexer = Lexer::new(&self.code);
        let tokens = lexer.lex();

        // parser
        let mut parser = Parser::new(tokens);
        let nodes = parser.parse();

        // interpreter
        let mut interpreter = interpreter::Interpreter::new();

        // todo: register external functions and environment variables

        interpreter.interpret(nodes);
    }
}