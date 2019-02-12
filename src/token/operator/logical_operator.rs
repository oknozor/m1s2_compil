use crate::token::operator::Operator;
use crate::token::logical_operator::LogOp::Or;
use crate::token::logical_operator::LogOp::And;
use crate::token::logical_operator::LogOp::None;

#[derive( Clone, PartialEq)]
pub enum LogOp {
    Or,
    And,
    None
}

impl Operator for LogOp {
    type OperatorKind = LogOp;

    fn from_str(str_op: &str) -> Self {
        match str_op {
            "||" => Or,
            "&&" => And,
            _=> None // FIXME
        }
    }

    fn as_str<'op>(&self) -> &'op str {
        match self {
            Or => "||",
            And => "&&",
            _ => "NaOp"
        }
    }
}

impl From<&String> for LogOp {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
            "||" => Or,
            "&&" => And,
            _ => None,
        }
    }
}
