use crate::token::operator::Operator;
use crate::token::binary_operator::BinaryOp::*;

#[derive(Clone, PartialEq)]
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
    PartialEq,
    StrictEq,
    None,
}

impl BinaryOp {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            LeftParenthesis => "(",
            RightParenthesis => ")",
            LessThan => "<",
            LessThanOrEq => "<=",
            GreaterThan => ">",
            GreaterThanOrEq => ">=",
            PartialEq => "!=",
            StrictEq => "==",
            None => "Nao"
        }
    }
}

impl From<&String> for BinaryOp {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
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
            "!=" => PartialEq,
            "==" => StrictEq,
            _ => None
        }
    }
}


impl BinaryOp {
    pub fn get_precedence(a: &BinaryOp, b: &BinaryOp) -> bool {
        match (a, b) {
            (Mul, _) => true,
            (Div, _) => true,
            (Mod, _) => true,
            _ => false,
        }
    }
}