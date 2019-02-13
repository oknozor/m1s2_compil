use crate::token::operator::operator::Operator;
use crate::token::operator::update_operator::UpdateOperator::*;

#[derive( Clone, PartialEq, Debug)]
pub enum UpdateOperator {
    Increment,
    Decrement,
    None
}

impl UpdateOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Increment => "++",
            Decrement => "--",
            _ => "NaOp"
        }
    }
}

impl From<&String> for UpdateOperator {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
            "--" => Decrement,
            "++" => Increment,
            _ => None,
        }
    }
}
