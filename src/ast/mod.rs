use std::iter::Peekable;
use thiserror::Error;
use crate::ast::expressions::parse_expression;
use crate::lexer::TokenIter;

pub mod expressions;
pub mod functions;

#[derive(Error, Debug, PartialEq)]
#[error("parse error: {:?}", msg)]
pub struct ParseError {
    msg: String,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Binary(Box<BinaryExpr>),
    Call(Box<CallExpr>),
    ProtoType(Box<ProtoTypeExpr>),
    Function(Box<FunctionExpr>),
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    op: char,
    lhs: Expression,
    rhs: Expression,
}

#[derive(Debug, PartialEq)]
pub struct CallExpr {
    callee: String,
    args: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct ProtoTypeExpr {
    name: String,
    args: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpr {
    proto: ProtoTypeExpr,
    body: Expression,
}

/// for user input
pub fn parse_top_level_expression(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    let inline_expression = parse_expression(tokens)?;

    Ok(Expression::Function(
        Box::new(
            FunctionExpr {
                proto: ProtoTypeExpr {
                    name: "".to_string(),
                    args: vec![],
                },
                body: inline_expression
            }
        )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extern() {
        let mut tokens = TokenIter::new("4.0").peekable();

        let expression = parse_top_level_expression(
            &mut tokens
        );

        assert_eq!(
            expression,
            Ok(
                Expression::Function(
                    Box::new(
                        FunctionExpr{
                            proto: ProtoTypeExpr {
                                name: "".to_string(),
                                args: vec![],
                            },
                            body: Expression::Number(4.0),
                        }
                    )
                )
            )
        );
    }
}

