// serde cannot untag or flatten a nested enum for now
// see : https://github.com/serde-rs/serde/issues/1402
// could be useful for literals


use crate::token::token::Token;
use crate::token::literal::Literal;
use crate::token::call::Call;
use crate::token::operator::binary_operator::BinaryOperator::RightParenthesis;
use crate::token::operator::binary_operator::BinaryOperator::LeftParenthesis;
use crate::token::operator::binary_operator::BinaryOperator;
use crate::token::operator::unary_operator::UnaryOperator;
use crate::token::operator::update_operator::UpdateOperator;
use crate::token::operator::assignement_operator::AssignmentOperator;
use crate::token::operator::operator::Operator::BinOp;
use crate::token::operator::operator::Operator::UpdateOp;

pub type TokenStream = Vec<Token>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
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

pub trait ToToken {
    fn to_token(&self) -> Vec<Token>;
}

impl ToToken for Box<Expression> {
    fn to_token(&self) -> Vec<Token> {
        match self {
            box Expression::BinaryExpression(bin_exp) => bin_exp.to_token(),
            box Expression::UnaryExpression(unary_exp) => unary_exp.to_token(),
            box Expression::NumericLiteral(numeric) => numeric.to_token(),
            box Expression::StringLiteral(string) => string.to_token(),
            box Expression::Identifier(identifier) => identifier.to_token(),
            box Expression::UpdateExpression(update) => update.to_token(),
            box Expression::CallExpression(call) => call.to_token(),
            box Expression::AssignmentExpression(assign) => assign.to_token(),
            box Expression::LogicalExpression(log) => log.to_token(),
            box Expression::MemberExpression(member) => member.to_token(),
            box Expression::CallExpression(call) => call.to_token(),
        }
    }
}

impl ToToken for BinaryExpression {
    fn to_token(&self) -> TokenStream {
        let mut token_stream = vec![];
        if let Some(extra) = &self.extra { token_stream.push(Token::Operator(BinOp(LeftParenthesis))); }
        token_stream.extend_from_slice(self.left.to_token().as_slice());
        let op: BinaryOperator = BinaryOperator::from(&self.operator);
        token_stream.push(Token::Operator(BinOp(op)));
        token_stream.extend_from_slice(self.right.to_token().as_slice());
        if let Some(extra) = &self.extra { token_stream.push(Token::Operator(BinOp(LeftParenthesis))); };
        token_stream
    }
}

impl ToToken for UnaryExpression {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let op = UnaryOperator::from(&self.operator);
        token_stream.extend_from_slice(self.argument.to_token().as_slice());
        token_stream
    }
}

impl ToToken for StringLiteral {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let token = Token::Literal(Literal::StringLiteral(self.value.clone()));
        token_stream.push(token);
        token_stream
    }
}

impl ToToken for NumericLiteral {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let token = Token::Literal(Literal::NumericLiteral(self.value));
        token_stream.push(token);
        token_stream
    }
}


impl ToToken for Identifier {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let token = Token::Idendifier(self.name.clone());
        token_stream.push(token);
        token_stream
    }
}

impl ToToken for UpdateExpression {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.argument.to_token().as_slice());
        let op = UpdateOperator::from(&self.operator);
        token_stream.push(Token::Operator(UpdateOp(op)));
        token_stream
    }
}

impl ToToken for AssignmentExpression {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.left.to_token().as_slice());
        let op = AssignmentOperator::from(&self.operator);
        token_stream.extend_from_slice(self.right.to_token().as_slice());
        token_stream
    }
}

impl ToToken for LogicalExpression {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.left.to_token().as_slice());
        let op = AssignmentOperator::from(&self.operator);
        token_stream.extend_from_slice(self.right.to_token().as_slice());
        token_stream
    }
}

impl ToToken for MemberExpression {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.property.to_token().as_slice());
        token_stream
    }
}

impl ToToken for CallExpression {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let callee = self.callee.to_token();
        let mut args = vec![];
        self.arguments.iter().for_each(|arg|
            args.extend_from_slice(arg.to_token().as_slice())
        );

        let call = Token::Function(Call { args, callee });

        token_stream.push(call);
        token_stream
    }
}

// TODO: Object

/*pub struct ObjectExpression {
    pub properties: [ Property ],
}*/

//todo: generic literal
/*pub struct Property {
    pub key: Result< <T: Literal>, Identifier>,
    pub value: Expression,
    pub kind: String,
}*/