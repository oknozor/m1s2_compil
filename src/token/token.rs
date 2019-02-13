use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use super::literal::Literal;
use std::fmt::Debug;
use crate::token::literal::Literal::NumericLiteral;
use crate::token::call::Call;
use crate::token::control_flow::ControlFlow;
use crate::token::control_flow::ControlFlow::*;
use crate::scope::scope::Scope;
use crate::token::operator::binary_operator::BinaryOperator;
use crate::token::operator::update_operator::UpdateOperator;
use crate::token::operator::assignement_operator::AssignmentOperator;
use crate::token::operator::logical_operator::LogicalOperator;
use crate::token::operator::operator::Operator;
use crate::token::operator::update_operator::UpdateOperator::*;
use crate::token::operator::operator::Operator::*;

#[derive(PartialEq, Clone)]
//CallToken
// ControlFlows
pub enum Token {
    Literal(Literal),
    Operator(Operator),
    //Todo: replace all Non terminal token with scopes: ControlFlow + Identifier
    Scope(Box<Scope>),
    Idendifier(String),
    Function(Call),
    ControlFlow(ControlFlow),
    Undefined,
}

impl Token {
    pub fn solve_binary_expression(a: Literal, b: Literal, op: Token) -> Token { //todo Result<Token, Err>

        if let Token::Operator(BinOp(binary_operator)) = op {
            match binary_operator {
                BinaryOperator::Add => Token::Literal(a + b),
                BinaryOperator::Sub => Token::Literal(a - b),
                BinaryOperator::Mul => Token::Literal(a * b),
                BinaryOperator::Div => Token::Literal(a / b),
                BinaryOperator::StrictEq => Token::Literal(Literal::from(a == b)),
                BinaryOperator::PartialEq => Token::Literal(Literal::from(a != b)),
                BinaryOperator::LessThan => Token::Literal(Literal::from(a < b)),
                BinaryOperator::LessThanOrEq => Token::Literal(Literal::from(a <= b)),
                BinaryOperator::GreaterThan => Token::Literal(Literal::from(a > b)),
                BinaryOperator::GreaterThanOrEq => Token::Literal(Literal::from(a >= b)),
                _ => {
                    panic!("unable to calculate binary expression {}, {}, {}", a.to_string(), b.to_string(), binary_operator.to_string())
                }
            }
        } else {
            //Todo :: error handling
            Token::Undefined
        }
    }

    pub fn update(a: Literal, op: Token) -> Token { //todo Result<Token, Err>
        if let Token::Operator(UpdateOp(update_operator)) = op {
            match update_operator {
                UpdateOperator::Increment => Token::Literal(a + NumericLiteral(1)),
                UpdateOperator::Decrement => Token::Literal(a - NumericLiteral(1)),
                _ => {
                    panic!("unable to calculate assignment {}, {}", a.to_string(), update_operator.to_string())
                }
            }
        } else {
            //Todo :: error handling
            Token::Undefined
        }
    }

    pub fn assign(a: Literal, b: Literal, op: Token) -> Token {
        if let Token::Operator(AssignOp(assign_operator)) = op {
            match assign_operator {
                AssignmentOperator::AddAssign => Token::Literal(a + b),
                AssignmentOperator::SubAssign => Token::Literal(a - b),
                AssignmentOperator::MulAssign => Token::Literal(a * b),
                AssignmentOperator::DivAssign => Token::Literal(a / b),
                AssignmentOperator::ModAssign => Token::Literal(a % b),
                _ => panic!("Unexpected assignment operator")
            }
        } else {
            //Todo :: error handling
            Token::Undefined
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
            Token::Operator(op) => write!(f, "{}", op.as_str()),
            Token::Undefined => write!(f, "Undefined"),
            Token::Idendifier(id) => write!(f, "{}", id),
            Token::Function(Call { args, callee }) => write!(f, "{:?} ({:?})", callee, args),
            Token::ControlFlow(Return) => write!(f, "Return"),
            Token::ControlFlow(Continue) => write!(f, "Continue"),
            Token::ControlFlow(Break) => write!(f, "Break"),
            Token::Scope(sc) => write!(f, "{:?}", sc)
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::Literal(literal) => write!(f, "{}", literal),
            Token::Operator(op) => write!(f, "{}", op.as_str()),
            Token::Undefined => write!(f, "{}", "Undefined"),
            Token::Idendifier(id) => write!(f, "{}", id),
            Token::Function(Call { args, callee }) => write!(f, "Call {:?} {:?}", callee, args),
            Token::ControlFlow(Return) => write!(f, "Return"),
            Token::ControlFlow(Continue) => write!(f, "Continue"),
            Token::ControlFlow(Break) => write!(f, "Break"),
            Token::Scope(sc) => write!(f, "{:?}", sc)
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