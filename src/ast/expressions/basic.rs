use std::iter::Peekable;

use crate::ast::expressions::{CallExpr, Expression};
use crate::ast::{expressions, ParseError};
use crate::lexer::{Token, TokenIter};

fn parse_number(number: f64) -> Expression {
    Expression::Number(number)
}

/// Converts '(expression)'
fn parse_parenthesis_expression(
    tokens: &mut Peekable<TokenIter>,
) -> Result<Expression, ParseError> {
    tokens.next(); // destroy open bracket

    let inner_expression = expressions::parse_expression(tokens)?;

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
                args.push(expressions::parse_expression(tokens)?);

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
pub fn parse_primary(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    match tokens.peek() {
        Some(&Token::Identifier(_)) => parse_identifier(tokens),
        Some(&Token::Number(number)) => Ok(parse_number(number)),
        Some(&Token::Operator('(')) => parse_parenthesis_expression(tokens),
        _ => Err(ParseError {
            msg: "Unknown token when expecting an expression".to_string(),
        }),
    }
}
