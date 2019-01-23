// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// or try : https://play.rust-lang.org/?gist=ab0ff6d5fc1998ca1c16e9477918d849
impl Literal for &StringLiteral{
    fn get_as_string(self) -> String {
        format!("\"{}\"", self.value.clone()) // fixme: need a global 'ast lifetime to use ref instead of heap data
    }
}
impl Literal for &NumericLiteral{
    fn get_as_string(self) -> String {
        self.value.to_string()
    }
}

// todo: learn macro! to generify this
// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
pub trait Literal {
    fn get_as_string(self) -> String;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral)
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
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
