use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use super::literal::Literal;
use std::fmt::Debug;
use crate::token::binary_operator::BinaryOp;
use crate::token::binary_operator::BinaryOp::*;
use crate::token::update_operator::UpdateOp;
use crate::token::update_operator::UpdateOp::*;
use crate::token::assignement_operator::AssignOp;
use crate::token::assignement_operator::AssignOp::*;
use crate::token::literal::Literal::NumericLiteral;
use crate::token::operator::Operator;
use crate::token::logical_operator::LogOp;

#[derive (PartialEq, Clone)]
//CallToken
// ControlFlows
pub enum Token {
    Literal(Literal),
    BinaryOp(BinaryOp),
    UpdateOp(UpdateOp),
    AssignOp(AssignOp),
    LogOp(LogOp),
    Idendifier(String),
    Undefined,
}

impl Token {
    pub fn calc(a: Literal, b: Literal, op: Token) -> Token { //todo Result<Token, Err>
        match op {
            Token::BinaryOp(Add) => Token::Literal(a + b),
            Token::BinaryOp(Sub) => Token::Literal(a - b),
            Token::BinaryOp(Mul) => Token::Literal(a * b),
            Token::BinaryOp(Div) => Token::Literal(a / b),
            Token::BinaryOp(StrictEq) => Token::Literal(Literal::from(a == b)),
            Token::BinaryOp(PartialEq) => Token::Literal(Literal::from(a != b)),
            Token::BinaryOp(LessThan) => Token::Literal(Literal::from(a < b)),
            Token::BinaryOp(LessThanOrEq) => Token::Literal(Literal::from(a <= b)),
            Token::BinaryOp(GreaterThan) => Token::Literal(Literal::from(a > b)),
            Token::BinaryOp(GreaterThanOrEq) => Token::Literal(Literal::from(a >= b)),
            _ => {
                panic!("unable to calculate binary expression {}, {}, {}", a.to_string(), b.to_string(), op.to_string())
            }
        }
    }

    pub fn update(a: Literal, op: Token) -> Token { //todo Result<Token, Err>
        match op {
            Token::UpdateOp(Increment) => Token::Literal(a + NumericLiteral(1)),
            Token::UpdateOp(Decrement) => Token::Literal(a - NumericLiteral(1)),
            _ => {
                panic!("unable to calculate assignment {}, {}", a.to_string(), op.to_string())
            }
        }
    }

    pub fn assign(a: Literal, b: Literal, op: Token) -> Token {
        match op {
            Token::AssignOp(AddAssign) => Token::Literal(a + b),
            Token::AssignOp(SubAssign) => Token::Literal(a - b),
            Token::AssignOp(MulAssign) => Token::Literal(a * b),
            Token::AssignOp(DivAssign) => Token::Literal(a / b),
            Token::AssignOp(ModAssign) => Token::Literal(a % b),
            _ => panic!("Unexpected assignment operator")
        }
    }

    #[allow(unused)]
    pub fn is_literal(&self) -> bool {
        match self {
            Token::Literal(l) => true,
            _ => false
        }
    }

    pub fn as_literal(&self) -> &Literal {
        match self {
            Token::Literal(l) => l,
            _ => &Literal::NullLiteral
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self {
            Token::Literal(b) => write!(f, "{}", b),
            Token::UpdateOp(u) => write!(f, "{}", u),
            Token::BinaryOp(o) => write!(f, "{}", o.as_str()),
            Token::AssignOp(o) => write!(f, "{}", o.as_str()),
            Token::LogOp(o) => write!(f, "{}", o.as_str()),
            Token::Undefined => write!(f, "Undefined"),
            Token::Idendifier(id) => write!(f, "{}", id),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::Literal(literal) => write!(f, "{}", literal),
            Token::UpdateOp(op) => write!(f, "{}", op),
            Token::BinaryOp(op) => write!(f, "{}", op),
            Token::AssignOp(op) => write!(f, "{}", op),
            Token::LogOp(op) => write!(f, "{}", op),
            Token::Undefined  => write!(f, "{}", "Undefined"),
            Token::Idendifier(id) => write!(f, "{}", id),
        }
    }
}

impl Into<BinaryOp> for Token {
    fn into(self) -> BinaryOp {
        match self {
            Token::BinaryOp(b) => b,
            _ => panic!("not a binary operator")
        }
    }
}

impl Into<AssignOp> for Token {
    fn into(self) -> AssignOp {
        match self {
            Token::AssignOp(b) => b,
            _ => panic!("not a binary operator")
        }
    }
}

impl Into<UpdateOp> for Token {
    fn into(self) -> UpdateOp {
        match self {
            Token::UpdateOp(b) => b,
            _ => panic!("not a binary operator")
        }
    }
}

#[test]
fn should_increment_token() {
    let operator = Token::UpdateOp(Increment);
    let literal = Literal::NumericLiteral(1);

    let result = Token::update(literal, operator);
    assert_eq!(result, Token::Literal(NumericLiteral(2)));
}