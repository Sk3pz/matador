use std::time::Duration;
use better_term::{Color, flush_styles};
use crate::lexer::Lexer;
use crate::parser::Node;

mod parser;
mod interpreter;
mod lexer;
mod literal;
mod postfix;
mod operator;

const TEST_CODE: &str = include_str!("../matador_tests/calculator.mtdr");

fn timed<F: FnOnce() -> R, R>(f: F) -> (R, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

fn main() {
    //println!("{}Lexing test code..", Color::BrightYellow);
    //flush_styles();
    let (tokens, lex_time) = timed(|| {
        let mut lexer = Lexer::new(TEST_CODE);
        lexer.lex()
    });
    //println!("{}Lexed in {}{:?}", Color::BrightGreen, Color::BrightYellow, lex_time);
    //println!("{}Parsing tokens..", Color::BrightYellow);
    let token_length = tokens.len();
    //flush_styles();
    let (nodes, parse_time) = timed(|| {
        let mut parser = parser::Parser::new(tokens);
        parser.parse()
    });
    //println!("{}Parsed in {}{:?}", Color::BrightGreen, Color::BrightYellow, parse_time);
    //Node::display(&nodes);
    //println!("{}Interpreting nodes..", Color::BrightYellow);
    //flush_styles();
    let (_, interpret_time) = timed(|| {
        let mut interpreter = interpreter::Interpreter::new();
        interpreter.interpret(nodes);
    });
    //println!("{}Interpreted in {}{:?}", Color::BrightGreen, Color::BrightYellow, interpret_time);
    println!("{}Ran code in {}{:?} {}with {}{} {}tokens", Color::BrightGreen, Color::BrightYellow,
             interpret_time + parse_time + lex_time
             , Color::BrightGreen, Color::BrightYellow, token_length, Color::BrightGreen);
    flush_styles();
}
