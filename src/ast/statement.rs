use crate::ast::expression::Expression;
use crate::ast::expression::Identifier;
use crate::ast::expression::Loc;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use crate::runner::context::RunnerOption;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockStatement {
    pub body: Vec<Box<Statement>>
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IfStatement {
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct ForStatement {
    pub init: Option<Expression>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct ReturnStatement {
    pub argument: Option<Expression>,
}



#[derive(Serialize, Deserialize, Clone
)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    // ES5 only, need to implement ES6 pattern
    // https://github.com/estree/estree/blob/master/es5.md#patterns
    pub params: Vec<Identifier>,
    pub body: BlockStatement,

}

#[derive(Serialize, Deserialize, Clone
)]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub kind: String,
    #[serde(skip_serializing_if = "RunnerOption::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone
)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Option<Expression>,
    #[serde(skip_serializing_if = "RunnerOption::with_loc")]
    pub loc: Loc,
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"start: {}:{}, end: {}:{}",
               self.start.line,
               self.start.column,
               self.end.line,
               self.end.column)
    }
}

