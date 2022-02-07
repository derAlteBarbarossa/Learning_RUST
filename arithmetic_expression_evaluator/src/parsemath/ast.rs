use super::token::Node;

use std::error;

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    match expr {
        Node::Number(i) => Ok(i),
        
        Node::Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),

        Node::Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),

        Node::Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),

        Node::Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),

        Node::Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),

        Node::Negative(expr1) => Ok(-(eval(*expr1)?)),

    }
}