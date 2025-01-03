use crate::expression::expression::{Expr, ExprVisitor};
use crate::scanner::token::Token;

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}
impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        format!("({} {} {})",
                operator.lexeme,
                left.accept(self),
                right.accept(self)
        )
    }

    fn visit_literal_expr(&mut self, value: &Token) -> String {
        match &value.literal {
            Some(literal) => literal.to_string(),
            None => value.lexeme.clone(),
        }
    }
}
