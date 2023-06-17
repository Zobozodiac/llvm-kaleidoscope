use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Operator(char),
}

pub struct TokenIter<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        get_token(&mut self.chars)
    }
}

fn remove_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&item) = chars.peek() {
        if item.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn get_token(chars: &mut Peekable<Chars>) -> Option<Token> {
    remove_whitespace(chars);

    match chars.next() {
        None => None,
        Some(last_char) if last_char.is_alphabetic() => {
            let mut identifier_str: String = last_char.to_string();

            while let Some(&item) = chars.peek() {
                if item.is_alphanumeric() {
                    identifier_str.push(item);
                    chars.next();
                } else {
                    break;
                }
            }

            match identifier_str.as_str() {
                "def" => Some(Token::Def),
                "extern" => Some(Token::Extern),
                _ => Some(Token::Identifier(identifier_str)),
            }
        }
        Some(last_char) if last_char.is_digit(10) | (last_char == '.') => {
            let mut num_str: String = last_char.to_string();

            while let Some(&item) = chars.peek() {
                if item.is_digit(10) | (item == '.') {
                    num_str.push(item);
                    chars.next();
                }
            }

            // TODO handle if get an incorrect format like '...23.45.5'
            Some(Token::Number(num_str.parse().unwrap()))
        }
        Some(last_char) if last_char == '#' => {
            while let Some(item) = chars.next() {
                if (item == '\n') | (item == '\r') {
                    break;
                }
            }

            get_token(chars)
        }
        Some(last_char) => Some(Token::Operator(last_char)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_whitespace() {
        assert_eq!(get_token(&mut " ".chars().peekable()), None);
    }

    #[test]
    fn test_get_toke_eof() {
        assert_eq!(get_token(&mut "".chars().peekable()), None);
    }

    #[test]
    fn test_get_token_def() {
        assert_eq!(get_token(&mut "def".chars().peekable()), Some(Token::Def));
    }

    #[test]
    fn test_get_token_extern() {
        assert_eq!(
            get_token(&mut "extern".chars().peekable()),
            Some(Token::Extern)
        );
    }

    #[test]
    fn test_get_token_num() {
        assert_eq!(
            get_token(&mut "1.23".chars().peekable()),
            Some(Token::Number(1.23))
        );
    }

    #[test]
    fn test_get_token_identifier() {
        assert_eq!(
            get_token(&mut "a".chars().peekable()),
            Some(Token::Identifier("a".to_string()))
        );
    }

    #[test]
    fn test_get_token_operator() {
        assert_eq!(
            get_token(&mut "+".chars().peekable()),
            Some(Token::Operator('+'))
        );
    }

    #[test]
    fn test_get_token_remainder() {
        let mut chars = "def( hello".chars().peekable();

        assert_eq!(get_token(&mut chars), Some(Token::Def));
        assert_eq!(chars.collect::<String>(), "( hello");
    }

    #[test]
    fn test_get_token_multiple() {
        let mut chars = "def( hello".chars().peekable();

        assert_eq!(get_token(&mut chars), Some(Token::Def));
        assert_eq!(get_token(&mut chars), Some(Token::Operator('(')));
        assert_eq!(
            get_token(&mut chars),
            Some(Token::Identifier("hello".to_string()))
        );
    }

    #[test]
    fn test_token_iter() {
        let chars = "def( hello".chars().peekable();
        let mut token_iter = TokenIter { chars };

        assert_eq!(token_iter.next(), Some(Token::Def));
        assert_eq!(token_iter.next(), Some(Token::Operator('(')));
        assert_eq!(token_iter.next(), Some(Token::Identifier("hello".to_string())));
        assert_eq!(token_iter.next(), None);
    }
}
