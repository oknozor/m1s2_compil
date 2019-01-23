#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    NumericLiteral(NumericLiteral)
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
