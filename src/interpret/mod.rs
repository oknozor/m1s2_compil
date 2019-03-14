use crate::token::token::Token;
use crate::token::token::Token::*;
use crate::token::token::Operator::*;
use crate::token::token::Operator;
use crate::token::token::Literal;
use std::error::Error;
use crate::token::token::Literal::NullLiteral;

pub mod interpreter;
pub mod interpret_visitor;

pub fn solve_postfix(postfix_expression: &mut Vec<Token>) -> Literal {
    let mut postfix_expression = postfix_expression.clone();
    let mut stack_lit:Vec<Literal> = vec![];
    let mut stack_op:Vec<Operator> = vec![];
    postfix_expression.reverse();

    while !postfix_expression.is_empty() {
        if let Some(LiteralToken(lit)) = postfix_expression.last() {
            stack_lit.push(lit.clone());
            postfix_expression.pop();
        };

        if let Some(OperatorToken(op)) = postfix_expression.last() {
            let a = stack_lit.pop().expect("stack should not be empty");
            let b = stack_lit.pop().expect("stack should not be empty");
            stack_lit.push(op.solve(&a, &b));
            postfix_expression.pop();
        };
    };
    stack_lit.pop().expect("stack should not be empty")
}