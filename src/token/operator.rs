

use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;
use crate::token::update_operator::UpdateOp;
use crate::token::binary_operator::BinaryOp;
use crate::token::assignement_operator::AssignOp;


pub trait Operator {
    type OperatorKind;
    fn from_str(str_op: &str) -> Self::OperatorKind;
    fn as_str<'op>(&self) -> &'op str;
}



impl Display for UpdateOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}
impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

impl Display for AssignOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}

