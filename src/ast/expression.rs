// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// could be useful for literals

use crate::ast::expression::Expression::*;

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
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumericLit {
    pub value: f64,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
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
pub struct ObjectExp {
    pub properties: Vec<Box<Property>>,
    #[serde(skip_serializing_if = "super::with_loc")]
    pub loc: Loc,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct Property {
    pub key: Box<Expression>,
    pub value: Box<Expression>,
    pub kind: Option<String>,
}

impl BinaryExp {
    pub fn has_idendifier(&self, left: &Option<&Id>, right: &Option<&Id>) -> bool {
        if left.is_some() || right.is_some() {
            true
        } else {
            false
        }
    }
    #[allow(unused)]
    pub fn has_parenthesis(&self) -> bool {
        if let Some(extra) = &self.extra {
            true
        } else {
            false
        }
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl ToString for NumericLit {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

impl ToString for StringLit {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}


// probably a more elegant way to do this(trait) but it will do for now
impl Expression {
    pub fn get_loc(&self) -> Loc{
         match self {
            BinaryExpression(exp) => exp.loc.clone(),
            UnaryExpression(exp)=> exp.loc.clone(),
            NumericLiteral(exp)=> exp.loc.clone(),
            StringLiteral(exp)=> exp.loc.clone(),
            Identifier(exp)=> exp.loc.clone(),
            UpdateExpression(exp)=> exp.loc.clone(),
            CallExpression(exp)=> exp.loc.clone(),
            AssignmentExpression(exp)=> exp.loc.clone(),
            LogicalExpression(exp)=> exp.loc.clone(),
            MemberExpression(exp)=> exp.loc.clone(),
            ObjectExpression(exp)=> exp.loc.clone()
        }
    }
}