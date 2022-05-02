/// Lexer for the `Lox` programming language
use std::collections::HashMap;
use std::fmt::Display;

use crate::Lox;

static KEYWORDS_PAIRS: &[(&str, TokenType)] = &[
    ("and", TokenType::And),
    ("class", TokenType::Class),
    ("else", TokenType::Else),
    ("false", TokenType::False),
    ("for", TokenType::For),
    ("fun", TokenType::Fun),
    ("if", TokenType::If),
    ("nil", TokenType::Nil),
    ("or", TokenType::Or),
    ("print", TokenType::Print),
    ("return", TokenType::Return),
    ("super", TokenType::Super),
    ("this", TokenType::This),
    ("true", TokenType::True),
    ("var", TokenType::Var),
    ("while", TokenType::While),
];

/// Type of Tokens existing in Lox
#[derive(Debug, Clone)]
enum TokenType {
    // Single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    typ: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(typ: TokenType, lexeme: String, line: usize) -> Self {
        Self { typ, lexeme, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.typ, self.lexeme)
    }
}

// FIXME(alvaro): Make this Scanner work with a single-pass Iterator
// over the tokens (with a peekable method) so that avoid unnecessary
// loops over the source characters (see `peek` and `advance`)
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: KEYWORDS_PAIRS.iter().cloned().collect(),
        }
    }

    pub fn scan_tokens(&mut self, interpreter: &Lox) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(interpreter);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
        &self.tokens
    }

    // FIXME(alvaro): This could probably be done in a `From` implementation?
    fn scan_token(&mut self, interpreter: &Lox) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '"' => self.string(interpreter),
            '!' => {
                if self.next_match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.next_match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.next_match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.next_match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.next_match('/') {
                    // A comment goes until the end of the line
                    while self.peek().map(|c| c != '\n').unwrap_or(false) {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            // FIXME(alvaro): We should try to coallesce a string of invalid characters into a
            // single error message
            c => interpreter.error(self.line, format!("Unexpected character '{}'", c).as_ref()),
        }
    }

    /// Try to consume a string literal
    fn string(&mut self, interpreter: &Lox) {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            interpreter.error(self.line, "Unterminated string");
            return;
        }
        // Consume the closing '"'
        self.advance();

        // Trim the surrounding quotes
        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(literal))
    }

    /// Consume a number literal
    fn number(&mut self) {
        while let Some(c) = self.peek() {
            if is_digit(c) {
                self.advance();
                continue;
            } else if c == '.' {
                let next_c = self.peek_next();
                if !next_c.map(is_digit).unwrap_or(false) {
                    break;
                }

                // Consume the '.'
                self.advance();

                // Consume the fractional part
                while let Some(c) = self.peek() {
                    if is_digit(c) {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
        let number = self.source[self.start..self.current]
            .parse::<f64>()
            .expect("it should be a valid number format");
        self.add_token(TokenType::Number(number))
    }

    /// Consume an identifier
    fn identifier(&mut self) {
        while let Some(c) = self.peek() {
            if is_alphanumeric(c) {
                self.advance();
            } else {
                break;
            }
        }
        let ident_text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(ident_text)
            .unwrap_or(&TokenType::Identifier)
            .clone();
        self.add_token(token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // FIXME(alvaro): This is very inefficient, and should instead use an
    // iterator over the characters
    fn advance(&mut self) -> char {
        let next_char = self.peek().expect("current should be a valid index");
        self.current += 1;
        next_char
    }

    fn add_token(&mut self, typ: TokenType) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(typ, text.to_string(), self.line);
        self.tokens.push(token);
    }

    fn next_match(&mut self, expected: char) -> bool {
        let next_matches = self.peek().map(|c| c == expected).unwrap_or(false);
        if next_matches {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
