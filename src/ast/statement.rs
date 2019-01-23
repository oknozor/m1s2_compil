use crate::ast::expression::Expression;

#[derive(Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}