use super::token::Node;
use super::token::Token;
use super::token::OperPrec;
use super::tokeniser::Tokeniser;

use std::convert::From;
use std::error::Error;

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl From<Box<dyn Error>> for ParseError {
    fn from(_evalerr: Box<dyn Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}

pub struct Parser<'a> {
    tokeniser: Tokeniser<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    // Public Interface
    pub fn new(expression: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokeniser::new(expression);

        let current_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid Character".into()))
        };

        Ok(Parser {
            tokeniser: lexer,
            current_token: current_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);

        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e)
        }
    }

    // Private Interface
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;

            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();

        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            },

            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            },

            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;

                self.check_paren(Token::RightParen)?;

                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;

                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }

                Ok(expr)
            },

            _ => Err(ParseError::UnableToParse("Unable to parse".into())),
        }
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::AddSub)?;

                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Subtract => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::AddSub)?;

                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Multiply => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::MulDiv)?;

                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Divide => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::MulDiv)?;

                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Caret => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::Power)?;

                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            },

            _ => Err(ParseError::InvalidOperator(format!("Please enter valid operator {:?}", self.current_token)))
        }
    }

    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;

            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!("Expected {:?}, Got {:?}",
                                        expected, self.current_token)))
        }
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokeniser.next() {
            Some(token) => token,
            None=> return Err(ParseError::InvalidOperator("Invalid Character".into())),
        };

        self.current_token = next_token;

        Ok(())
    }
}
