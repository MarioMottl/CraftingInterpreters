use crate::expression::expression::Expr;
use crate::scanner::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.term()
    }

    // term -> factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    // factor -> primary ( ( "/" | "*" ) primary )*
    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.primary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    // primary -> NUMBER | "(" expression ")"
    fn primary(&mut self) -> Option<Expr> {
        if self.match_tokens(&[TokenType::Number]) {
            return Some(Expr::Literal {
                value: self.previous().clone(),
            });
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Some(expr);
        }

        None
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<&Token> {
        if self.check(token_type) {
            Some(self.advance())
        } else {
            println!("Error: {}", message);
            None
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}