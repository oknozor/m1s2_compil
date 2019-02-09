// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// could be useful for literals


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    NumericLiteral(JSLiteral<i64>),
    StringLiteral(JSLiteral<String>),
    Identifier(Identifier),
    UpdateExpression(UpdateExpression),
    CallExpression(CallExpression),
    AssignmentExpression(AssignmentExpression),
    LogicalExpression(LogicalExpression),
    MemberExpression(MemberExpression),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Identifier {
    pub name: String,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumericLiteral {
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateExpression {
    pub operator: String,
    pub argument: Box<Expression>,
    prefix: bool,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
    pub extra: Option<Extra>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Extra {
    pub parenthesized: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnaryExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssignmentExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicalExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Loc {
    pub start: Pos,
    pub end: Pos,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pos {
    pub line: i64,
    pub column: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct JSLiteral<T> {
    pub value: T
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