use crate::token::operator::binary_operator::BinaryOperator::*;
use crate::token::operator::operator::Operator;

#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOperator {
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
impl BinaryOperator {
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

    pub fn get_precedence(a: &BinaryOperator, b: &BinaryOperator) -> bool {
        match (a, b) {
            (Mul, _) => true,
            (Div, _) => true,
            (Mod, _) => true,
            _ => false,
        }
    }
}

impl From<&String> for BinaryOperator {
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