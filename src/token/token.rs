use std::collections::HashMap;

pub struct Node {
    tokens: Vec<Token>
}

#[derive(PartialEq, Clone)]
pub enum Token {
    LiteralToken(Literal),
    OperatorToken(Operator),
    IdendifierToken(String),
    FunctionToken(Call),
    Undefined,
}

#[derive(Clone)]
pub enum Literal {
    StringLiteral(String),
    NumericLiteral(f64),
    BooleanLiteral(bool),
    NullLiteral,
    Infinity,
}

#[derive (PartialEq, Clone, Debug, Copy)]
pub enum Operator {
    UnaryOp(UnaryOperator),
    LogOp(LogicalOperator),
    UpdateOp(UpdateOperator),
    BinOp(BinaryOperator),
    AssignOp(AssignmentOperator),
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum BinaryOperator {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
    PartialEq,
    StrictEq,
}

#[derive( Clone, PartialEq, Debug, Copy)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum UnaryOperator {
    Plus,
    Minus,
    ExPoint,
    Tilde,
    TypeOf,
    Void,
    Delete,
}

#[derive( Clone, PartialEq, Debug, Copy)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum AssignmentOperator {
    AddAssign,
    SubAssign,
    DivAssign,
    MulAssign,
    ModAssign,
}

#[derive (PartialEq, Clone)]
pub struct Call {
    pub args: Vec<Token>,
    pub callee: String,
}