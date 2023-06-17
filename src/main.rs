use std::io;
use std::io::Write;
use std::iter::Peekable;

use crate::ast::functions::{parse_extern, parse_function};
use crate::ast::parse_top_level_expression;
use crate::lexer::{Token, TokenIter};

mod ast;
mod lexer;

fn handle_function(tokens: &mut Peekable<TokenIter>) {
    println!("Parsed a function definition.");
    parse_function(tokens).expect("Error parsing function!");
}

fn handle_extern(tokens: &mut Peekable<TokenIter>) {
    println!("Parsed en extern.");
    parse_extern(tokens).expect("Error parsing extern!");
}

fn handle_top_level_expression(tokens: &mut Peekable<TokenIter>) {
    println!("Parsed a top-level expression.");
    parse_top_level_expression(tokens).expect("Error parsing top-level expression!");
}

fn main() {
    loop {
        print!("ready: ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Error reading input");

        let mut tokens = TokenIter::new(&line).peekable();

        match tokens.peek() {
            None => {
                break;
            },
            Some(&Token::Operator(';')) => {
                // Just ignore semicolons and move on
                tokens.next();
            },
            Some(&Token::Def) => {
                handle_function(&mut tokens);
            },
            Some(&Token::Extern) => {
                handle_extern(&mut tokens);
            },
            _ => {
                handle_top_level_expression(&mut tokens);
            }
        }
    }
}
