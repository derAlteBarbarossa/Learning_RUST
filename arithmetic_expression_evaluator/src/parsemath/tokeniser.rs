// This module reads characters from an arithmetic expression
// and translates them into token
use super::token::Token;

use std::str::Chars;
use std::iter::Peekable;


pub struct Tokeniser<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokeniser<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokeniser {
            expr: new_expr.chars().peekable(),
        }
    }

}

impl<'a> Iterator for Tokeniser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0' ..= '9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            },
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(_) => None,
            None => Some(Token::EOF),
        }
    }
}

