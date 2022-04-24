/// Lexer for the `Lox` programming language

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn scan(&self) -> Vec<String> {
        self.source
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }
}
