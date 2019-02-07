use crate::ast::token::Token;

#[derive (PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub rpn_exp: Vec<Token>,
}