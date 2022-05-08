mod expr;
/// Interpreter for the Lox programming language from the
/// "Crafting Interpreters" book
mod lexer;

use std::fmt::{Debug, Display};
use std::io::BufRead;

use lexer::Scanner;

// TODO(alvaro): Look into `thiserror` for hanlding this boilerplate
#[derive(Debug)]
enum LoxError {
    IOError(std::io::Error),
    Generic(String),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::IOError(e) => write!(f, "IOError: {}", e),
            LoxError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for LoxError {
    fn from(e: std::io::Error) -> Self {
        LoxError::IOError(e)
    }
}

impl From<String> for LoxError {
    fn from(e: String) -> Self {
        LoxError::Generic(e)
    }
}

#[derive(Debug)]
pub struct Lox {}

impl Lox {
    fn new() -> Self {
        Self {}
    }

    fn run_file(&self, script_name: String) -> Result<(), LoxError> {
        println!("Running from script {}", script_name);
        let contents = std::fs::read_to_string(script_name)?;
        match self.run(contents) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.error(0, format!("{}", err).as_ref());
                Ok(())
            }
        }
    }

    fn run_prompt(&self) -> Result<(), LoxError> {
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

fn main() -> Result<(), LoxError> {
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
        _ => return Err("usage: rinlox [script]".to_string().into()),
    }

    Ok(())
}
