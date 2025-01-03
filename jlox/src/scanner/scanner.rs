use crate::ast::ast_printer::AstPrinter;
use crate::parser::parser::Parser;
use crate::scanner::token::{Token, TokenType};
use std::cmp::PartialEq;

pub struct Scanner {
    source: String,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = self.source.chars().peekable();
        let mut line = 1;

        while let Some(&_c) = chars.peek() {
            match self.scan_token(&mut chars, &mut line) {
                Ok(Some(token)) => {
                    if token.token_type != TokenType::Eof {
                        tokens.push(token);
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    crate::errors::generic_error::error(line, &e);
                    break;
                }
            }
            chars.next(); // Consume the character
        }

        tokens.push(Token::new(TokenType::Eof, "".to_string(), None, line));
        tokens
    }

    pub fn scan_token(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line: &mut usize,
    ) -> Result<Option<Token>, String> {
        while let Some(&c) = chars.peek() {
            match c {
                '(' => return Ok(Some(self.make_token(TokenType::LeftParen, chars, line))),
                ')' => return Ok(Some(self.make_token(TokenType::RightParen, chars, line))),
                '{' => return Ok(Some(self.make_token(TokenType::LeftBrace, chars, line))),
                '}' => return Ok(Some(self.make_token(TokenType::RightBrace, chars, line))),
                ',' => return Ok(Some(self.make_token(TokenType::Comma, chars, line))),
                '.' => return Ok(Some(self.make_token(TokenType::Dot, chars, line))),
                '-' => return Ok(Some(self.make_token(TokenType::Minus, chars, line))),
                '+' => return Ok(Some(self.make_token(TokenType::Plus, chars, line))),
                ';' => return Ok(Some(self.make_token(TokenType::Semicolon, chars, line))),
                '*' => return Ok(Some(self.make_token(TokenType::Star, chars, line))),
                '!' => {
                    return Ok(Some(self.match_token(
                        '=',
                        TokenType::BangEqual,
                        TokenType::Bang,
                        chars,
                        line,
                    )))
                }
                '=' => {
                    return Ok(Some(self.match_token(
                        '=',
                        TokenType::EqualEqual,
                        TokenType::Equal,
                        chars,
                        line,
                    )))
                }
                '<' => {
                    return Ok(Some(self.match_token(
                        '=',
                        TokenType::LessEqual,
                        TokenType::Less,
                        chars,
                        line,
                    )))
                }
                '>' => {
                    return Ok(Some(self.match_token(
                        '=',
                        TokenType::GreaterEqual,
                        TokenType::Greater,
                        chars,
                        line,
                    )))
                }
                '/' => {
                    chars.next();
                    if let Some('/') = chars.peek() {
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    } else {
                        return Ok(Some(Token::new(
                            TokenType::Slash,
                            "/".to_string(),
                            None,
                            *line,
                        )));
                    }
                }
                '\n' => {
                    *line += 1;
                    chars.next();
                }
                ' ' | '\r' | '\t' => {
                    chars.next();
                }
                '"' => return Ok(Some(self.string(chars, line))),
                _ => {
                    return if c.is_alphabetic() {
                        Ok(Some(self.identifier(chars, line)))
                    } else if c.is_ascii_digit() {
                        Ok(Some(self.number(chars, line)))
                    } else {
                        Err(format!("Unexpected character: {}", c))
                    }
                }
            }
        }
        Ok(None)
    }

    fn match_token(
        &self,
        expected: char,
        if_match: TokenType,
        if_not: TokenType,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line: &mut usize,
    ) -> Token {
        let mut lexeme = chars.next().unwrap().to_string();
        if let Some(&next) = chars.peek() {
            if next == expected {
                lexeme.push(chars.next().unwrap());
                return Token::new(if_match, lexeme, None, *line);
            }
        }
        Token::new(if_not, lexeme, None, *line)
    }

    fn make_token(
        &self,
        token_type: TokenType,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line: &mut usize,
    ) -> Token {
        let lexeme = chars.next().unwrap().to_string();
        Token::new(token_type, lexeme, None, *line)
    }

    fn identifier(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        line: &mut usize,
    ) -> Token {
        let mut lexeme = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_alphanumeric() {
                lexeme.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        Token::new(token_type, lexeme, None, *line)
    }

    fn string(&self, chars: &mut std::iter::Peekable<std::str::Chars>, line: &mut usize) -> Token {
        let mut lexeme = String::new();
        chars.next(); // Consume the opening quote
        while let Some(&c) = chars.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                *line += 1;
            }
            lexeme.push(chars.next().unwrap());
        }
        chars.next(); // Consume the closing quote
        Token::new(TokenType::String, lexeme, None, *line)
    }

    fn number(&self, chars: &mut std::iter::Peekable<std::str::Chars>, line: &mut usize) -> Token {
        let mut lexeme = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                lexeme.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        Token::new(TokenType::Number, lexeme, None, *line)
    }
}

pub fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.clone());
    if let Some(expression) = parser.parse() {
        let mut printer = AstPrinter::new();
        println!("\nAST: {}", printer.print(&expression));
    }

    // Keep token printing for debugging
    println!("\nTokens:");
    for token in tokens {
        println!("{:?}", token);
    }
}
