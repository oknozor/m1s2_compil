use crate::token::operator::assignement_operator::AssignmentOperator::*;

#[derive(Clone, PartialEq, Debug)]
pub enum AssignmentOperator {
    AddAssign,
    SubAssign,
    DivAssign,
    MulAssign,
    ModAssign,
    None,
}

impl AssignmentOperator {
    pub fn as_str<'op>(&self) -> &'op str {
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

impl From<&String> for AssignmentOperator {
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