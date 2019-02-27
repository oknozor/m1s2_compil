use crate::ast::expression::Loc;

pub mod statement;
pub mod expression;

#[allow(unused)]
pub fn with_loc(loc: &Loc) -> bool {
    true
}
