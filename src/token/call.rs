use crate::token::token::Token;

#[derive (PartialEq, Clone)]
pub struct Call {
    pub args: Vec<Token>,
    pub callee: Vec<Token>,
}
