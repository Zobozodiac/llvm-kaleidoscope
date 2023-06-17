use std::iter::Peekable;

use binary::parse_binary_operation;

use crate::ast::{Expression, ParseError};
use crate::ast::expressions::basic::parse_primary;
use crate::lexer::{Token, TokenIter};

pub mod basic;
pub mod binary;

/// This is the main work behind expressions, such as 'a + (b * get_mean(x))'.
pub fn parse_expression(tokens: &mut Peekable<TokenIter>) -> Result<Expression, ParseError> {
    let initial_expression = parse_primary(tokens)?;

    parse_binary_operation(tokens, initial_expression, 0)
}

#[cfg(test)]
mod tests {
    use crate::ast::BinaryExpr;
    use super::*;

    #[test]
    fn test_parse_expression() {
        let mut tokens = TokenIter::new("x+y").peekable();

        let expression = parse_expression(&mut tokens);

        assert_eq!(
            expression,
            Ok(
                Expression::Binary(
                    Box::new(
                        BinaryExpr {
                            op: '+',
                            lhs: Expression::Variable("x".to_string()),
                            rhs: Expression::Variable("y".to_string()),
                        }
                    )
                )
            )
        );
    }

    #[test]
    fn test_parse_expression_number() {
        let mut tokens = TokenIter::new("4.0").peekable();

        let expression = parse_expression(&mut tokens);

        assert_eq!(
            expression,
            Ok(Expression::Number(4.0))
        );
    }
}
