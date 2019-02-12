use crate::token::operator::Operator;
use crate::token::unary_operator::UnaryOp::*;

#[derive(Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    ExPoint,
    Tilde,
    TypeOf,
    Void,
    Delete,
    None
}

impl UnaryOp {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Plus => "+",
            Minus => "-",
            ExPoint => "!",
            Tilde => "~",
            TypeOf => "void",
            Void => "typeof",
            Delete => "delete",
            _ => "Nao"
        }
    }
}

impl From<&String> for UnaryOp {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
            "+" => Plus,
            "-" => Minus,
            "!" => ExPoint,
            "~" => Tilde,
            "void" => Void,
            "typeof" => TypeOf,
            "delete" => Delete,
            _ => None,
        }
    }
}