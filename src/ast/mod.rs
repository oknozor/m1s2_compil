use crate::ast::expression::Loc;

pub mod node;
pub mod statement;
pub mod expression;

pub fn with_loc(loc: &Loc) -> bool {
    true
}
