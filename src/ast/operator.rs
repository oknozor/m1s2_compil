use crate::ast::literal::Number;
use crate::ast::operator::BinaryOp::*;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;

#[derive( Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    LeftParenthesis,
    RightParenthesis,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
}

pub enum UnaryOp {
    IncrementPost,
    IncrementPre,
    DecrementPost,
    DecrementPre,
    Minus,
    Plus,
    Not,
}
pub enum BitOp {
    And,
    Or,
    Xor,
    ShiftL,
    ShiftR,
}

impl BinaryOp {
    pub fn from_str(str_op: &str) -> Self {
        match str_op {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            "%" => Mod,
            "(" => LeftParenthesis,
            ")" => RightParenthesis,
            "<" => LessThan,
            "<=" => LessThanOrEq,
            ">" => GreaterThan,
            ">=" => GreaterThanOrEq,
            _ => Add
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            LeftParenthesis => "(",
            RightParenthesis => ")",
            LessThan => "<" ,
            LessThanOrEq => "<=",
            GreaterThan => ">" ,
            GreaterThanOrEq => ">=",
        }
    }

    pub fn get_precedence(a: &BinaryOp, b: &BinaryOp) -> bool {
        match (a, b)  {
            (Mul, Add) => true,
            (Mul, Sub) => true,
            (Div, Add) => true,
            (Div, Sub) => true,
            (Mod, Add) => true,
            (Mod, Mod) => true,
            _ => false,
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str());
        Ok(())
    }
}

