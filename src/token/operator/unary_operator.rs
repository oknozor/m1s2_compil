use crate::token::operator::unary_operator::UnaryOperator::*;

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
    ExPoint,
    Tilde,
    TypeOf,
    Void,
    Delete,
    None,
}

impl UnaryOperator {
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

impl From<&String> for UnaryOperator {
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