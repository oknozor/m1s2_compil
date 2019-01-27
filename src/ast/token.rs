use crate::ast::literal::Number;
use crate::ast::operator::BinaryOp;
use std::ops::Add;
use crate::ast::token::Token::*;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use crate::runner::standard_lib::Function;

#[derive(Clone)]
pub enum Token {
    BooleanLiteral(bool),
    NullLiteral,
    NumericLiteral(Number),
    StringLiteral(String),
    RegularExpression(String),
    BinaryOp(BinaryOp),
    BinOp,
    Undefined,
    StandardFunction(Function)
}

impl Add<Token> for Token {
    type Output = Token;

    fn add(self, other: Token) -> Self::Output {
        match (self, other) {
            (NumericLiteral(a), NumericLiteral(b)) => Token::NumericLiteral(a + b),
            /*            (Token::NumOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::NumOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Token::BinOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::BinOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),*/
            _ => panic!("Unkown error")
        }
    }
}

impl Sub<Token> for Token {
    type Output = Token;

    fn sub(self, other: Token) -> Self::Output {
        match (self, other) {
            (NumericLiteral(a), NumericLiteral(b)) => Token::NumericLiteral(a - b),
            /*            (Token::NumOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::NumOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Token::BinOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::BinOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),*/
            _ => panic!("Unkown error")
        }
    }
}


impl Mul for Token {
    type Output = Token;

    fn mul(self, other: Token) -> Self::Output {
        match (self, other) {
            (NumericLiteral(a), NumericLiteral(b)) => Token::NumericLiteral(a * b),
            /*            (Token::NumOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::NumOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Token::BinOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::BinOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),*/
            _ => panic!("Unkown error")
        }
    }
}

impl Div for Token {
    type Output = Token;

    fn div(self, other: Token) -> Self::Output {
        match (self, other) {
            (NumericLiteral(a), NumericLiteral(b)) => Token::NumericLiteral(a / b),
            /*           (Token::NumOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::NumOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Token::BinOp(n), Null) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),
                        (Null, Token::BinOp(n)) => panic!("Invalid left hand side expression, exprected a literal go \"{}\" ", n.to_string()),*/
            _ => panic!("Unkown error")
        }
    }
}

impl Token {
    pub fn calc(a: Token, b: Token, op: Token) -> Token {
        match op {
            Token::BinaryOp(BinaryOp::Add) => a + b,
            Token::BinaryOp(BinaryOp::Sub) => a - b,
            Token::BinaryOp(BinaryOp::Mul) => a * b,
            Token::BinaryOp(BinaryOp::Div) => a / b,
            _ => panic!("badabom")
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Token::BooleanLiteral(b) => write!(f, "{}", b),
            Token::NullLiteral => write!(f, "null"),
            Token::NumericLiteral(n) => write!(f, "{}",n),
            Token::StringLiteral(s) => write!(f,"{}",s),
            Token::RegularExpression(s) => write!(f,"{}",s),
            Token::RegularExpression(s) => write!(f,"{}",s),
            Token::BinaryOp(o) => write!(f, "{}", o.as_str()),
            Token::BinOp => write!(f,"|"),
            Token::Undefined => write!(f, "Undefined"),
            Token::StandardFunction(func) => write!(f, "Function call"),
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
