// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// or try : https://play.rust-lang.org/?gist=ab0ff6d5fc1998ca1c16e9477918d849

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    UpdateExpression(UpdateExpression),
    CallExpression(CallExpression),
    AssignmentExpression(AssignmentExpression),
    EmptyStatement,
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
    pub argument: Box<Expression>, // not sure boxing is the most efficient way to handle recursive struct
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
    // todo: Operator enum
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub operator: String,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

// todo: could make a proc macro to generify this
// see : https://doc.rust-lang.org/stable/book/ch19-06-macros.html
pub trait Literal {
    fn get_as_string(self) -> String;
}

impl Literal for &StringLiteral {
    // fixme: need a global 'ast lifetime to use ref instead of heap data
    fn get_as_string(self) -> String {
        format!("\"{}\"", self.value.clone())
    }
}

impl Literal for &NumericLiteral {
    fn get_as_string(self) -> String {
        self.value.to_string()
    }
}