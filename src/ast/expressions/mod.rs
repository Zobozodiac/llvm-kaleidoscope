use std::iter::Peekable;

use binary::parse_binary_operation;

use crate::ast::ParseError;
use crate::lexer::{Token, TokenIter};

pub mod basic;
pub mod binary;

pub enum Expression {
    Number(f64),
    Variable(String),
    Binary(Box<BinaryExpr>),
    Call(Box<CallExpr>),
}

pub struct BinaryExpr {
    op: char,
    lhs: Expression,
    rhs: Expression,
}

struct CallExpr {
    callee: String,
    args: Vec<Expression>,
}

/// This is the main work behind expressions, such as 'a + (b * get_mean(x))'.
fn parse_expression(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    let initial_expression = parse_expression(tokens)?;

    parse_binary_operation(tokens, initial_expression, 0)
}
