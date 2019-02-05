use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use crate::ast::operator::BinaryOp;

use super::literal::Literal;

#[derive(Clone)]
pub enum Token {
    Literal(Literal),
    BinaryOp(BinaryOp),
    BinOp,
    Undefined,
}

impl Token {
    pub fn calc(a: Literal, b: Literal, op: Token) -> Token {
        match op {
            Token::BinaryOp(BinaryOp::Add) => Token::Literal(a + b),
            Token::BinaryOp(BinaryOp::Sub) => Token::Literal(a - b),
            Token::BinaryOp(BinaryOp::Mul) => Token::Literal(a * b),
            Token::BinaryOp(BinaryOp::Div) => Token::Literal(a / b),
            Token::BinaryOp(BinaryOp::LessThan) => Token::Literal(Literal::from(a < b)),
            _ => panic!("badabom")
        }
    }

    #[allow(unused)]
    pub fn is_literal(&self) -> bool {
        match self {
            Token::Literal(l) => true,
            _ => false
        }
    }

    pub fn as_literal(&self) -> &Literal {
        match self {
            Token::Literal(l) => l,
            _ => &Literal::NullLiteral
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Token::Literal(b) => write!(f, "{}", b),
            Token::BinaryOp(o) => write!(f, "{}", o.as_str()),
            Token::BinOp => write!(f, "|"),
            Token::Undefined => write!(f, "Undefined"),
        }
    }
}

impl Into<BinaryOp> for Token {
    fn into(self) -> BinaryOp {
        match self {
            Token::BinaryOp(b) => b,
            _ => panic!("not a binary operator")
        }
    }
}
