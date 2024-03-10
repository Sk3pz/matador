use std::time::Duration;
use better_term::{Color, flush_styles, Style};
use crate::lexer::Lexer;

mod parser;
mod node;
mod interpreter;
mod lexer;
mod variable;
mod postfix;
mod operator;
mod scope;


const TEST_CODE: &str = include_str!("../matador_tests/array.mtdr");

fn timed<F: FnOnce() -> R, R>(f: F) -> (R, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

fn main() {
    println!("{}Matador {}v0.1ALPHA",
             Style::new().fg(Color::Cyan).bold(),
             Style::new().overwrite().fg(Color::BrightCyan));
    flush_styles();
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
    //println!("{}Interpreting nodes..", Color::BrightYellow);
    //flush_styles();
    let (_, interpret_time) = timed(|| {
        let mut interpreter = interpreter::Interpreter::new();
        interpreter.interpret(nodes);
    });
    //println!("{}Interpreted in {}{:?}", Color::BrightGreen, Color::BrightYellow, interpret_time);
    println!("{gb}Ran code in {y}{:?} {gb}with {y}{} {gb}tokens.",
             interpret_time + parse_time + lex_time, token_length,
             y = Style::new().overwrite().fg(Color::BrightYellow),
             gb = Style::new().fg(Color::BrightGreen).bold());
    flush_styles();
}
