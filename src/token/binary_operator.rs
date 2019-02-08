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
    None
}

impl Operator for BinaryOp {
    type OperatorKind = BinaryOp;

    fn from_str(str_op: &str) -> Self {
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
            "!=" => PartialEq,
            "==" => StrictEq,
            _ => None
        }
    }
    fn as_str<'op>(&self) -> &'op str {
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