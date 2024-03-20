use std::io::{stdin, stdout, Write};

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    loop {
        print!("calc> ");

        // Flush stdout to ensure prompt is displayed immediately
        stdout().flush().unwrap();

        // Read user input as a String
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::new(&mut parser);
        let result = interpreter.interpret();
        match result {
            Ok(value) => println!("{}", value),
            Err(e) => println!("{}", e),
        }
    }
}
