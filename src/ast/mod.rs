use std::iter::Peekable;
use crate::lexer::{Token, TokenIter};
use thiserror::Error;
use crate::ast::Expression::Variable;

enum Expression {
    Number(f64),
    Variable(String),
    Binary(Box<BinaryExpr>),
    Call(Box<CallExpr>)
}

struct BinaryExpr {
    op: char,
    lhs: Expression,
    rhs: Expression,
}

struct CallExpr {
    callee: String,
    args: Vec<Expression>
}

struct ProtoType {
    name: String,
    args: Vec<Expression>
}

struct Function {
    proto: ProtoType,
    body: Expression,
}

#[derive(Error, Debug)]
#[error("parse error: {:?}", msg)]
pub struct ParseError {
    msg: String
}

fn parse_number(number: f64) -> Expression {
    Expression::Number(number)
}

/// Converts '(expression)'
fn parse_parenthesis_expression(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    tokens.next();  // destroy open bracket

    let inner_expression = parse_expression(tokens)?;

    match tokens.next() {
        Some(Token::Operator(')')) => (),
        _ => {
            return Err(ParseError { msg: "Expected ')'".to_string() })
        }
    };

    Ok(inner_expression)
}


fn parse_expression(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    todo!()
}

/// Converts text starting with identifier such as 'data' or 'get_data(a, b)'.
fn parse_identifier(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    let variable = match tokens.next() {
        Some(Token::Identifier(identifier)) => {
            identifier
        },
        _ => return Err(ParseError{ msg: "Expected Identifier Token".to_string() })
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
                    },
                    Some(Token::Operator(',')) => {
                        ()
                    },
                    _ => {
                        return Err(
                            ParseError { msg: "Expected  ',' or ')'".to_string() }
                        )
                    }
                }
            }

            Ok(Expression::Call(
                Box::new(
                    CallExpr {
                        callee: variable,
                        args,
                    }
                )
            ))
        },
        _ => Ok(Expression::Variable(variable))
    }
}

fn parse_primary(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    match tokens.peek() {
        Some(&Token::Identifier(_)) => {
            parse_identifier(tokens)
        },
        Some(&Token::Number(number)) => {
            Ok(parse_number(number))
        },
        Some(&Token::Operator('(')) => {
            parse_parenthesis_expression(tokens)
        },
        _ => {
            Err(ParseError { msg: "Unknown token when expecting an expression".to_string()})
        }
    }
}


