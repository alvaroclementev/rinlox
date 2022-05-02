/// Interpreter for the Lox programming language from the
/// "Crafting Interpreters" book
mod lexer;
mod expr;
    
use std::error::Error;
use std::fmt::{Debug, Display};
use std::io::BufRead;

use lexer::Scanner;

#[derive(Debug, Clone)]
struct LoxError(String);

// TODO(alvaro): Learn how to do this error handling properly

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for LoxError {}

#[derive(Debug)]
pub struct Lox {}

impl Lox {
    fn new() -> Self {
        Self {}
    }

    fn run_file(&self, script_name: String) -> Result<(), String> {
        println!("Running from script {}", script_name);
        let contents = std::fs::read_to_string(script_name).map_err(|e| format!("{}", e))?;
        match self.run(contents) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.error(0, format!("{}", err).as_ref());
                Ok(())
            }
        }
    }

    fn run_prompt(&self) -> Result<(), String> {
        println!("Running from prompt");
        let stdin = std::io::stdin();
        for line in stdin.lock().lines().flatten() {
            if let Err(err) = self.run(line) {
                self.error(0, format!("{}", err).as_ref());
            }
        }
        Ok(())
    }

    fn run(&self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens(self);
        for (i, token) in scanner.tokens.iter().enumerate() {
            println!("Token {}: {}", i, token)
        }
        Ok(())
    }

    fn error(&self, line: usize, msg: &str) {
        self.report(line, "", msg)
    }

    fn report(&self, line: usize, loc_str: &str, msg: &str) {
        println!("[line {}] Error{}: {}", line, loc_str, msg);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args();

    match args.len() {
        1 => {
            let lox = Lox::new();
            lox.run_prompt()?
        }
        2 => {
            let lox = Lox::new();
            lox.run_file(args.into_iter().nth(1).unwrap())?
        }
        _ => return Err("usage: rinlox [script]".into()),
    }

    Ok(())
}
