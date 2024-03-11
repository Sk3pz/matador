use std::time::Duration;
use better_term::{Color, flush_styles, Style};
use crate::lexer::Lexer;
use crate::variable::Variable;

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

const TEST_CODE: &str = include_str!("../matador_tests/general.mtdr");

pub const DEBUG_OUTPUT: bool = true;

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
    let (tokens, lex_time) = timed(|| {
        let mut lexer = Lexer::new(TEST_CODE);
        lexer.lex()
    });
    let token_length = tokens.len();
    let (nodes, parse_time) = timed(|| {
        let mut parser = parser::Parser::new(tokens);
        parser.parse()
    });
    let (_, interpret_time) = timed(|| {
        let mut interpreter = interpreter::Interpreter::new();
        interpreter.register_native_function("print_native", |args| {
            if args.len() > 1 {
                // error
                println!("{} Print only takes 1 argument!", Color::Red);
                std::process::exit(1);
            } else if args.len() == 1 {
                println!("{}", args[0]);
            } else {
                println!("{}", args[0]);
            }
            Variable::Int(0)
        });
        interpreter.interpret(nodes);
    });
    println!("{gb}Ran code in {y}{:?} {gb}with {y}{} {gb}tokens.",
             interpret_time + parse_time + lex_time, token_length,
             y = Style::new().overwrite().fg(Color::BrightYellow),
             gb = Style::new().fg(Color::BrightGreen).bold());
    flush_styles();
}
