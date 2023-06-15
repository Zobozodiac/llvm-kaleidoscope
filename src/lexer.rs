use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
    Operator(char),
}

fn get_token(mut chars: &mut Peekable<Chars>) -> Token {
    remove_whitespace(&mut chars);

    match chars.next() {
        None => Token::EOF,
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
                "def" => Token::Def,
                "extern" => Token::Extern,
                _ => Token::Identifier(identifier_str),
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
            Token::Number(num_str.parse().unwrap())
        }
        Some(last_char) if last_char == '#' => {
            while let Some(item) = chars.next() {
                if (item == '\n') | (item == '\r') {
                    break;
                }
            }

            get_token(chars)
        }
        Some(last_char) => Token::Operator(last_char),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        assert_eq!(get_token(&mut "".chars().peekable()), Token::EOF);
        assert_eq!(get_token(&mut " ".chars().peekable()), Token::EOF);
        assert_eq!(get_token(&mut "def".chars().peekable()), Token::Def);
        assert_eq!(get_token(&mut "extern".chars().peekable()), Token::Extern);
        assert_eq!(
            get_token(&mut "1.23".chars().peekable()),
            Token::Number(1.23)
        );
        assert_eq!(
            get_token(&mut "a".chars().peekable()),
            Token::Identifier("a".to_string())
        );
        assert_eq!(get_token(&mut "+".chars().peekable()), Token::Operator('+'));
    }

    #[test]
    fn test_get_token_remainder() {
        let mut chars = "def( hello".chars().peekable();

        assert_eq!(get_token(&mut chars), Token::Def);
        assert_eq!(chars.collect::<String>(), "( hello");
    }

    #[test]
    fn test_get_token_multiple() {
        let mut chars = "def( hello".chars().peekable();

        assert_eq!(get_token(&mut chars), Token::Def);
        assert_eq!(get_token(&mut chars), Token::Operator('('));
        assert_eq!(get_token(&mut chars), Token::Identifier("hello".to_string()));
    }
}
