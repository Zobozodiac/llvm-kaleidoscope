use crate::lexer::{Token, TokenIter};
use std::iter::Peekable;
use thiserror::Error;
use crate::ast::binary::parse_binary_operation;

pub mod binary;

pub enum Expression {
    Number(f64),
    Variable(String),
    Binary(Box<BinaryExpr>),
    Call(Box<CallExpr>),
}

struct BinaryExpr {
    op: char,
    lhs: Expression,
    rhs: Expression,
}

struct CallExpr {
    callee: String,
    args: Vec<Expression>,
}

struct ProtoType {
    name: String,
    args: Vec<Expression>,
}

struct Function {
    proto: ProtoType,
    body: Expression,
}

#[derive(Error, Debug)]
#[error("parse error: {:?}", msg)]
pub struct ParseError {
    msg: String,
}

fn parse_number(number: f64) -> Expression {
    Expression::Number(number)
}

/// Converts '(expression)'
fn parse_parenthesis_expression(
    tokens: &mut Peekable<TokenIter>,
) -> Result<Expression, ParseError> {
    tokens.next(); // destroy open bracket

    let inner_expression = parse_expression(tokens)?;

    match tokens.next() {
        Some(Token::Operator(')')) => (),
        _ => {
            return Err(ParseError {
                msg: "Expected ')'".to_string(),
            })
        }
    };

    Ok(inner_expression)
}

/// Converts text starting with identifier such as 'data' or 'get_data(a, b)'.
fn parse_identifier(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    let variable = match tokens.next() {
        Some(Token::Identifier(identifier)) => identifier,
        _ => {
            return Err(ParseError {
                msg: "Expected Identifier Token".to_string(),
            })
        }
    };

    match tokens.peek() {
        Some(&Token::Operator('(')) => {
            tokens.next();
            let mut args = vec![];

            loop {
                args.push(parse_expression(tokens)?);

                match tokens.next() {
                    Some(Token::Operator(')')) => {
                        break;
                    }
                    Some(Token::Operator(',')) => (),
                    _ => {
                        return Err(ParseError {
                            msg: "Expected  ',' or ')'".to_string(),
                        })
                    }
                }
            }

            Ok(Expression::Call(Box::new(CallExpr {
                callee: variable,
                args,
            })))
        }
        _ => Ok(Expression::Variable(variable)),
    }
}

/// A primary expression is one which is just a single identifier, something enclosed in brackets, or a number.
fn parse_primary(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    match tokens.peek() {
        Some(&Token::Identifier(_)) => parse_identifier(tokens),
        Some(&Token::Number(number)) => Ok(parse_number(number)),
        Some(&Token::Operator('(')) => parse_parenthesis_expression(tokens),
        _ => Err(ParseError {
            msg: "Unknown token when expecting an expression".to_string(),
        }),
    }
}

/// This is the main work behind expressions, such as 'a + (b * get_mean(x))'.
fn parse_expression(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    let initial_expression = parse_expression(tokens)?;

    parse_binary_operation(tokens, initial_expression, 0)
}
