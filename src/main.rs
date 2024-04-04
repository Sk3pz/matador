use better_term::{Color, flush_styles, Style};
use crate::debug::timed;
use crate::lexer::Lexer;
use crate::node::Node;

mod parser;
mod node;
mod interpreter;
mod lexer;
mod variable;
mod postfix;
mod operator;
mod scope;
pub mod debug;
mod function;
mod matador_std;

const TEST_CODE: &str = include_str!("../matador_tests/brainfuck2.mtdr");

pub const DEBUG_OUTPUT: bool = false;

fn main() {
    println!("{}Matador {}v0.1ALPHA",
             Style::new().fg(Color::Cyan).bold(),
             Style::new().overwrite().fg(Color::BrightCyan));
    flush_styles();
    let (tokens, lex_time) = timed(|| {
        let mut lexer = Lexer::new(TEST_CODE);
        lexer.lex()
    });
    let token_length = tokens.len();
    let (nodes, parse_time) = timed(|| {
        let mut parser = parser::Parser::new(tokens);
        parser.parse()
    });
    if DEBUG_OUTPUT {
        Node::prgm_display(&nodes);
    }
    let (_, interpret_time) = timed(|| {
        let mut interpreter = interpreter::Interpreter::new();
        matador_std::attach_std(&mut interpreter);
        interpreter.interpret(nodes);
    });
    println!("{gb}Ran code in {y}{:?} {gb}with {y}{} {gb}tokens.",
             interpret_time + parse_time + lex_time, token_length,
             y = Style::new().overwrite().fg(Color::BrightYellow),
             gb = Style::new().fg(Color::BrightGreen).bold());
    flush_styles();
}
