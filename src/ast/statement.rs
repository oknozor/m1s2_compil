use crate::ast::expression::Expression;
use crate::ast::expression::Identifier;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Statement {
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    WhileStatement(WhileStatement),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarator(VariableDeclarator),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
}

#[derive(Serialize, Deserialize)]
pub struct BlockStatement {
    pub body: Vec<Box<Statement>>
}

#[derive(Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Serialize, Deserialize)]
pub struct IfStatement {
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Box<Option<Statement>>,
}

#[derive(Serialize, Deserialize)]
pub struct ForStatement {
    pub init: Option<Expression>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Serialize, Deserialize)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Serialize, Deserialize)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

#[derive(Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
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

