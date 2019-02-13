use crate::token::token::Token;
use crate::token::token::Token::*;
use crate::token::literal::Literal::*;
use crate::ast::expression::TokenStream;
use crate::token::call;
use crate::token::operator::operator::Operator;
use crate::token::operator::operator::Operator::BinOp;

/**
while there are tokens to be read:
    read a token.
    if the token is a number, then:
        push it to the output queue.
    if the token is a function then:
        push it onto the operator stack
    if the token is an operator, then:
        while ((there is a function at the top of the operator stack)
               or (there is an operator at the top of the operator stack with greater precedence)
               or (the operator at the top of the operator stack has equal precedence and is left associative))
              and (the operator at the top of the operator stack is not a left bracket):
            pop operators from the operator stack onto the output queue.
        push it onto the operator stack.
    if the token is a left bracket (i.e. "("), then:
        push it onto the operator stack.
    if the token is a right bracket (i.e. ")"), then:
        while the operator at the top of the operator stack is not a left bracket:
            pop the operator from the operator stack onto the output queue.
        pop the left bracket from the stack.
        /* if the stack runs out without finding a left bracket, then there are mismatched parentheses. */
if there are no more tokens to read:
    while there are still operator tokens on the stack:
        /* if the operator token on the top of the stack is a bracket, then there are mismatched parentheses. */
        pop the operator from the operator stack onto the output queue.
exit.
**/

impl Stack {
    pub fn postfix_and_solve(stream: TokenStream) {
        stream.iter().for_each(|token| {
            match token {
                Literal(NumericLiteral(num)) => (),
                Function(call::Call { args, callee }) => (),
                Token::Operator(BinOp( operator) )=> (),
                _=> ()
            }
        })
    }
}

pub struct Stack {
    pub queue: Vec<Token>,
    pub operators: Vec<Operator>,
}


impl Stack {
    pub fn new() -> Self {
        Stack {
            queue: vec![],
            operators: vec![],
        }
    }
}
