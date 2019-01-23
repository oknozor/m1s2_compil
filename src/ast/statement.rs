use crate::ast::expression::Expression;

#[derive(Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub kind: String,
}

#[derive(Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Option<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}



