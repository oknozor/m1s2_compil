use crate::token::operator::Operator;
use crate::token::update_operator::UpdateOp::Increment;
use crate::token::update_operator::UpdateOp::Decrement;
use crate::token::update_operator::UpdateOp::None;

#[derive( Clone, PartialEq)]
pub enum UpdateOp {
    Increment,
    Decrement,
    None
}

impl Operator for UpdateOp {
    type OperatorKind = UpdateOp;

    fn from_str(str_op: &str) -> Self {
        match str_op {
            "++" => Increment,
            "--" => Decrement,
            _=> Increment // FIXME
        }
    }

    fn as_str<'op>(&self) -> &'op str {
        match self {
            Increment => "++",
            Decrement => "--",
            _ => "NaOp"
        }
    }
}

impl From<&String> for UpdateOp {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
            "--" => Decrement,
            "++" => Increment,
            _ => None,
        }
    }
}
