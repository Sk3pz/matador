use better_term::{Color, read_input};
use crate::interpreter::Interpreter;
use crate::variable::Variable;

pub fn attach_std(interpreter: &mut Interpreter) {
    // register the standard functions

    // === PRINT ===
    interpreter.register_native_function("print", |args| {
        if args.len() > 1 {
            // error
            print!("{} Print only takes 1 argument!", Color::Red);
        } else if args.len() == 1 {
            print!("{}", args[0]);
        }
        Variable::Int(0)
    });

    interpreter.register_native_function("println", |args| {
        if args.len() > 1 {
            // error
            println!("{} Print only takes 1 argument!", Color::Red);
        } else if args.len() == 1 {
            println!("{}", args[0]);
        } else {
            println!();
        }
        Variable::Int(0)
    });

    // === READ ===
    interpreter.register_native_function("readln", |args| {
        if args.len() > 0 {
            // error
            println!("{} readln takes no arguments", Color::Red);
        }
        let input = read_input!();
        Variable::String(input)
    });

    interpreter.register_native_function("readint", |args| {
        if args.len() > 0 {
            // error
            println!("{} readint takes no arguments", Color::Red);
        }
        let input = read_input!();
        Variable::Int(input.parse().unwrap_or_else(|_| {
            println!("{} Invalid input, expected an integer", Color::Red);
            std::process::exit(1);
        }))
    });

    interpreter.register_native_function("readbool", |args| {
        if args.len() > 0 {
            // error
            println!("{} readbool takes no arguments", Color::Red);
        }
        let input = read_input!();
        Variable::Bool(input.parse().unwrap_or_else(|_| {
            println!("{} Invalid input, expected a boolean", Color::Red);
            std::process::exit(1);
        }))
    });

    interpreter.register_native_function("readfloat", |args| {
        if args.len() > 0 {
            // error
            println!("{} readfloat takes no arguments", Color::Red);
        }
        let input = read_input!();
        Variable::Float(input.parse().unwrap_or_else(|_| {
            println!("{} Invalid input, expected a float", Color::Red);
            std::process::exit(1);
        }))
    });

    // === CHAR ===
    interpreter.register_native_function("to_ascii_char", |args| {
        if args.len() > 1 {
            // error
            println!("{} to_ascii_char takes 1 argument", Color::Red);
        }
        let int = match args[0] {
            Variable::Int(i) => i,
            _ => {
                println!("{} to_ascii_char takes an integer", Color::Red);
                std::process::exit(1);
            }
        };
        let c = int as u8 as char;
        Variable::String(c.to_string())
    });
}