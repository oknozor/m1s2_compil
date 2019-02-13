use crate::token::operator::logical_operator::LogicalOperator::*;

#[derive( Clone, PartialEq, Debug)]
pub enum LogicalOperator {
    Or,
    And,
    None
}

impl LogicalOperator {
    pub fn as_str<'op>(&self) -> &'op str {
        match self {
            Or => "||",
            And => "&&",
            _ => "NaOp"
        }
    }
}

impl From<&String> for LogicalOperator {
    fn from(string: &String) -> Self {
        let op = string.as_str();
        match op {
            "||" => Or,
            "&&" => And,
            _ => None,
        }
    }
}
