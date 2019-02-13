use crate::ast::expression::Expression;
use crate::ast::expression::Identifier;
use crate::ast::expression::Loc;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use crate::ast::expression::Pos;

pub trait Named {
    fn get_name(&self) -> String;
}
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    SwitchCase(SwitchCase),
    ForStatement(ForStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    ReturnStatement(ReturnStatement),
    EmptyStatement,
    Root(Vec<Statement>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<Statement>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwitchStatement {
    pub discriminant: Box<Expression>,
    pub cases: Vec<Box<Statement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwitchCase {
    pub test: Option<Box<Expression>>,
    pub consequent: Vec<Box<Statement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IfStatement {
    pub test: Box<Expression>,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForStatement {
    pub init: Option<Box<Expression>>,
    pub test: Option<Box<Expression>>,
    pub update: Option<Box<Expression>>,
    pub body: Box<Statement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhileStatement {
    pub test: Box<Expression>,
    pub body: Box<Statement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReturnStatement {
    pub argument: Option<Box<Expression>>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    // ES5 only, need to implement ES6 pattern
    // https://github.com/estree/estree/blob/master/es5.md#patterns
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VariableDeclaration {
    pub declarations: Vec<Box<Statement>>,
    pub kind: String,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
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

impl Statement {
    pub fn get_name(&self) -> Option<String> {
        match self {
            Statement::FunctionDeclaration(ref func) => Some(func.id.name.clone()),
            Statement::VariableDeclarator(ref v) => Some(v.id.name.clone()),
            _ => None
        }
    }
}

#[test]
fn should_get_name() {
    let function = FunctionDeclaration {
        id: Identifier {
            name: "luke".to_string(),
            loc: Loc {
                start: Pos { line: 0, column: 0 },
                end: Pos { line: 0, column: 0 },
            },
        },
        params: vec![],
        body: BlockStatement {
            body: vec![Box::new(Statement::EmptyStatement)]
        },
    };

    let statement = Statement::FunctionDeclaration(function);

    assert_eq!("luke", statement.get_name().unwrap());
}



