use crate::scanner::token::Token;


pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        // Token scanning logic goes here
        // For now, return an empty vector
        vec![]
    }
}

pub fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // For now, just print the tokens.
    for token in tokens {
        println!("{:?}", token);
    }
}
