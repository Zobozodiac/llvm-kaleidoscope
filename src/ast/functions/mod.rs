use std::iter::Peekable;

use crate::ast::{Expression, FunctionExpr, ParseError, ProtoTypeExpr};
use crate::ast::expressions::parse_expression;
use crate::lexer::{Token, TokenIter};

fn parse_prototype(tokens: &mut Peekable<TokenIter>)  -> Result<ProtoTypeExpr, ParseError> {
    match tokens.next() {
        Some(Token::Identifier(identifier)) => {
            match tokens.next() {
                Some(Token::Operator('(')) => {
                    let mut args = vec![];

                    loop {
                        match tokens.next() {
                            Some(Token::Identifier(arg)) => {
                                args.push(arg);
                            },
                            Some(Token::Operator(')')) => {
                                return Ok(ProtoTypeExpr {
                                    name: identifier,
                                    args,
                                })
                            },
                            _ => return Err(ParseError { msg: "Expected ')' in prototype".to_string()})
                        }
                    }
                },
                _ => return Err(
                    ParseError { msg: "Expected '(' in prototype".to_string()}
                )
            }
        },
        _ => return Err(ParseError {msg: "Expected function name in prototype".to_string()})
    }
}

pub fn parse_function(tokens: &mut Peekable<TokenIter>)  -> Result<Expression, ParseError> {
    tokens.next(); // consume def

    Ok(
        Expression::Function(Box::new(FunctionExpr {
            proto: parse_prototype(tokens)?,
            body: parse_expression(tokens)?,
        }))
    )
}

pub fn parse_extern(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    tokens.next(); // consume extern

    Ok(Expression::ProtoType(
        Box::new(
            parse_prototype(tokens)?
        )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extern() {
        let mut tokens = TokenIter::new("extern sin(arg)").peekable();

        let expression = parse_extern(
            &mut tokens
        );

        assert_eq!(
            expression,
            Ok(
                Expression::ProtoType(
                    Box::new(
                        ProtoTypeExpr {
                            name: "sin".to_string(),
                            args: vec![
                                "arg".to_string()
                            ]
                        }
                    )
                )
            )
        );
    }

    #[test]
    fn test_parse_function_simple() {
        let mut tokens = TokenIter::new("def constant() 4.0").peekable();

        let expression = parse_function(
            &mut tokens
        );

        assert_eq!(
            expression,
            Ok(
                Expression::Function(
                    Box::new(
                        FunctionExpr {
                            proto: ProtoTypeExpr {
                                name: "constant".to_string(),
                                args: vec![]
                            },
                            body: Expression::Number(4.0),
                        }
                    )
                )
            )
        );
    }
}
