

use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;
use crate::token::operator::update_operator::UpdateOperator;
use crate::token::operator::binary_operator::BinaryOperator;
use crate::token::operator::logical_operator::LogicalOperator;
use crate::token::operator::assignement_operator::AssignmentOperator;
use crate::token::operator::unary_operator::UnaryOperator;
use crate::token::operator::operator::Operator::*;

// FIXME Generic operator method
#[derive (PartialEq, Clone, Debug)]
pub enum Operator {
    UnaryOp(UnaryOperator),
    LogOp(LogicalOperator),
    UpdateOp(UpdateOperator),
    BinOp(BinaryOperator),
    AssignOp(AssignmentOperator)
}


// TO DO: Debug, Display
impl Operator {
    pub fn as_str(&self) -> &str {
        match self {
            UnaryOp(o) => o.as_str(),
            LogOp(o) => o.as_str(),
            UpdateOp(o) => o.as_str(),
            BinOp(o) => o.as_str(),
            AssignOp(o) => o.as_str(),
        }
    }
}

impl Display for UpdateOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}
impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}
impl Display for LogicalOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}
impl Display for AssignmentOperator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

