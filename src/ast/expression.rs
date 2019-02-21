// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// could be useful for literals

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExp),
    UnaryExpression(UnaryExp),
    NumericLiteral(NumericLit),
    StringLiteral(StringLit),
    Identifier(Id),
    UpdateExpression(UpdateExp),
    CallExpression(CallExp),
    AssignmentExpression(AssignmentExp),
    LogicalExpression(LogicalExp),
    MemberExpression(MemberExp),
    ObjectExpression(ObjectExp),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Id {
    pub name: String,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StringLit {
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumericLit {
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateExp {
    pub operator: String,
    pub argument: Box<Expression>,
    prefix: bool,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CallExp {
    pub callee: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BinaryExp {
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
pub struct UnaryExp {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssignmentExp {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogicalExp {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemberExp {
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
    pub line: f64,
    pub column: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct JSLiteral<T> {
    pub value: T
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct ObjectExp {
    pub properties: Vec<Box<Property>>
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct Property {
    pub key: Box<Expression>,
    pub value: Box<Expression>,
    pub kind: Option<String>,
}