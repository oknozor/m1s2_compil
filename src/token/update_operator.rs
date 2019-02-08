use crate::token::operator::Operator;
use crate::token::update_operator::UpdateOp::Increment;
use crate::token::update_operator::UpdateOp::Decrement;

#[derive( Clone, PartialEq)]
pub enum UpdateOp {
    Increment,
    Decrement,
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
            Decrement => "--"
        }
    }
}
