use crate::token::operator::Operator;
use crate::token::assignement_operator::AssignOp::*;

#[derive(Clone, PartialEq)]
pub enum AssignOp {
    AddAssign,
    SubAssign,
    DivAssign,
    MulAssign,
    ModAssign,
    None,
}

impl Operator for AssignOp {
    type OperatorKind = AssignOp;

    fn from_str(str_op: &str) -> Self {
        match str_op {
            "+=" => AddAssign,
            "-=" => SubAssign,
            "/=" => DivAssign,
            "*=" => MulAssign,
            "%=" => ModAssign,
            _ => None
        }
    }
    fn as_str<'op>(&self) -> &'op str {
        match self {
            AddAssign => "+=",
            SubAssign => "-=",
            DivAssign => "/=",
            MulAssign => "*=",
            ModAssign => "%=",
            None => "NaOp"
        }
    }
}

impl From<&String> for AssignOp {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
            "+=" => AddAssign,
            "-=" => SubAssign,
            "/=" => DivAssign,
            "*=" => MulAssign,
            "%=" => ModAssign,
            _ => None
        }
    }
}