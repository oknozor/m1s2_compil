use crate::ast::expression::Expression;
use crate::ast::expression::Id;
use crate::ast::expression::Loc;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::cell::RefCell;

pub trait Named {
    fn get_name(&self) -> String;
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Statement {
    BlockStatement(BlockStmt),
    ExpressionStatement(ExpressionStmt),
    WhileStatement(WhileStmt),
    VariableDeclaration(VariableDec),
    VariableDeclarator(Variable),
    FunctionDeclaration(FunctionDec),
    IfStatement(IfStmt),
    SwitchStatement(SwitchStmt),
    SwitchCase(CaseStmt),
    ForStatement(ForStmt),
    BreakStatement(BreakStmt),
    ContinueStatement(ContinueStmt),
    ReturnStatement(ReturnStmt),
    EmptyStatement,
    Root(Vec<Statement>),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootStatement {
    Program(Program),
    File(File),
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub program: Program,
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Box<Statement>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockStmt {
    pub body: Vec<Box<Statement>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionStmt {
    pub expression: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwitchStmt {
    pub discriminant: Box<Expression>,
    pub cases: Vec<Box<CaseStmt>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaseStmt {
    pub test: Option<Box<Expression>>,
    pub consequent: Vec<Box<Statement>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IfStmt {
    pub test: Box<Expression>,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForStmt {
    pub init: Option<Box<Expression>>,
    pub test: Option<Box<Expression>>,
    pub update: Option<Box<Expression>>,
    pub body: Box<Statement>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhileStmt {
    pub test: Box<Expression>,
    pub body: Box<Statement>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BreakStmt {
    pub label: Option<Id>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReturnStmt {
    pub argument: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContinueStmt {
    pub label: Option<Id>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionDec {
    pub id: Id,
    pub params: Vec<Id>,
    pub body: BlockStmt,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VariableDec {
    pub declarations: Vec<Box<Statement>>,
    pub kind: String,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Variable {
    pub id: Id,
    pub init: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
    #[serde(default = "delay")]
    pub delayed:  RefCell<bool>,
}

pub fn delay() -> RefCell<bool> {
    RefCell::new(true)
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        self.id.name.clone()
    }
}

impl RootStatement {
    pub fn get_program_root(&self) -> Option<Vec<Box<Statement>>> {
        match self {
            RootStatement::Program(p) => Some( p.body.to_owned()),
            RootStatement::File(ref f) => Some(f.program.body.to_owned()),
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "start: {}:{}, end: {}:{}",
               self.start.line,
               self.start.column,
               self.end.line,
               self.end.column)
    }
}



