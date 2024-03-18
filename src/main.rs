use std::io::{stdin, stdout, Write};

mod token;
mod interpreter;
mod lexer;

use interpreter::Interpreter;
use lexer::Lexer;

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
        let mut interpreter = Interpreter::new(&mut lexer);
        let result = interpreter.expr();
        match result {
            Ok(value) => println!("{}", value),
            Err(e) => println!("{}", e),
        }
    }
}

