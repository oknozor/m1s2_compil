// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// could be useful for literals

use crate::ast::literal::Literal;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    NumericLiteral(Literal<i64>),
    StringLiteral(Literal<String>),
    Identifier(Identifier),
    UpdateExpression(UpdateExpression),
    CallExpression(CallExpression),
    AssignmentExpression(AssignmentExpression),
    LogicalExpression(LogicalExpression),
    MemberExpression(MemberExpression),
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct NumericLiteral {
    pub value: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpression {
    pub operator: String,
    pub argument: Box<Expression>,
    // not sure boxing is the most efficient way to handle recursive struct
    //todo
    prefix: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

#[derive(Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
    pub extra: Option<Extra>
}

#[derive(Serialize, Deserialize)]
pub struct Extra {
    pub parenthesized: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct LogicalExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

/*pub struct ObjectExpression {
    pub properties: [ Property ],
}*/

//todo: generic literal
/*pub struct Property {
    pub key: Result< <T: Literal>, Identifier>,
    pub value: Expression,
    pub kind: String,
}*/