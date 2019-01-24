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
    FunctionDeclaration(FunctionDeclaration),
    IfStatement(IfStatement),
    SwitchStatement(SwitchStatement),
    ForStatement(ForStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    ReturnStatement(ReturnStatement),
    EmptyStatement,
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
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Serialize, Deserialize)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
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
pub struct ReturnStatement {
    pub argument: Option<Expression>,
}



#[derive(Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    // ES5 only, need to implement ES6 pattern
    // https://github.com/estree/estree/blob/master/es5.md#patterns
    pub params: Vec<Identifier>,
    pub body: BlockStatement,

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

