use std::iter::Peekable;
use std::ops::Index;

use crate::ast::expressions::basic::parse_primary;
use crate::ast::{BinaryExpr, Expression, ParseError};
use crate::lexer::{Token, TokenIter};

/// Binary operators in increasing order of importance
static BINARY_OPERATORS: [char; 3] = ['-', '+', '*'];

fn is_binary_operator(token: Option<&Token>) -> bool {
    match token {
        Some(Token::Operator(operator)) if BINARY_OPERATORS.contains(operator) => true,
        _ => false,
    }
}

fn get_binary_operator_precedence(token: &Token) -> Option<usize> {
    match token {
        Token::Operator(operator) if BINARY_OPERATORS.contains(operator) => {
            BINARY_OPERATORS.iter().position(|a| a == operator)
        }
        _ => None,
    }
}

fn get_operator_char(token: &Token) -> Option<char> {
    match token {
        Token::Operator(operator) => Some(*operator),
        _ => None,
    }
}

pub fn parse_binary_operation(
    tokens: &mut Peekable<TokenIter>,
    lhs: Expression,
    cutoff_precedence: usize,
) -> Result<Expression, ParseError> {
    let mut lhs = lhs;

    loop {
        let operator = tokens.peek();

        if !is_binary_operator(operator) {
            return Ok(lhs);
        }

        let operator_token = tokens.next().unwrap();

        let operator_precedence = get_binary_operator_precedence(&operator_token).unwrap();

        if operator_precedence < cutoff_precedence {
            return Ok(lhs);
        }

        let operator_char = get_operator_char(&operator_token).unwrap();

        let next_expression = parse_primary(tokens)?;

        let next_operator = tokens.peek();

        if !is_binary_operator(next_operator) {
            return Ok(Expression::Binary(Box::new(BinaryExpr {
                op: operator_char,
                lhs,
                rhs: next_expression,
            })));
        }

        let next_operator_precedence =
            get_binary_operator_precedence(&next_operator.unwrap()).unwrap();

        if next_operator_precedence < operator_precedence {
            lhs = Expression::Binary(Box::new(BinaryExpr {
                op: operator_char,
                lhs,
                rhs: next_expression,
            }))
        } else {
            lhs = Expression::Binary(Box::new(BinaryExpr {
                op: operator_char,
                lhs,
                rhs: parse_binary_operation(tokens, next_expression, operator_precedence)?,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression::Binary;

    #[test]
    fn test_parse_binary_operation() {
        let mut tokens = TokenIter::new("+b");
        let initial_expression = Expression::Variable("a".to_string());

        let expression = parse_binary_operation(&mut tokens.peekable(), initial_expression, 0);

        assert_eq!(
            expression,
            Ok(Binary(Box::new(BinaryExpr {
                op: '+',
                lhs: Expression::Variable("a".to_string()),
                rhs: Expression::Variable("b".to_string()),
            })))
        )
    }
}
