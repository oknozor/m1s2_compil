use std::collections::HashMap;

use crate::ast::expression::Expression::*;
use crate::ast::expression::*;
use crate::ast::statement::Statement;
use crate::ast::statement::Statement::*;
use crate::token::token::*;
use crate::token::token::Operator::*;
use crate::token::token::Token::*;
use crate::visitor::Visitor;
use crate::token::Precedence;

#[derive(Debug)]
pub struct Stack {
    pub out_queue: Vec<Token>,
    pub op_queue: Vec<Token>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            out_queue: vec![],
            op_queue: vec![],
        }
    }
}

// TODO : return slice
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
            box Expression::ObjectExpression(object) => object.to_token(),
        }
    }
}

impl ToToken for BinaryExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        /*if let Some(extra) = &self.extra*/
        let op: BinaryOperator = BinaryOperator::from(self.operator.as_str());
        let left_postfix = postfix(&mut self.left.to_token());
        let left_tokens = postfix(&mut self.right.to_token());
        token_stream
    }
}

impl ToToken for UnaryExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let op = UnaryOperator::from(self.operator.as_str());
        token_stream.extend_from_slice(self.argument.to_token().as_slice());
        token_stream
    }
}

impl ToToken for StringLit {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let token = Token::LiteralToken(Literal::StringLiteral(self.value.clone()));
        token_stream.push(token);
        token_stream
    }
}

impl ToToken for NumericLit {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let token = Token::LiteralToken(Literal::NumericLiteral(self.value));
        token_stream.push(token);
        token_stream
    }
}


impl ToToken for Id {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let token = Token::IdendifierToken(self.name.clone());
        token_stream.push(token);
        token_stream
    }
}

impl ToToken for UpdateExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.argument.to_token().as_slice());
        let op = UpdateOperator::from(self.operator.as_str());
        token_stream.push(Token::OperatorToken(UpdateOp(op)));
        token_stream
    }
}

impl ToToken for AssignmentExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.left.to_token().as_slice());
        let op = AssignmentOperator::from(self.operator.as_str());
        token_stream.extend_from_slice(self.right.to_token().as_slice());
        token_stream
    }
}

impl ToToken for LogicalExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.left.to_token().as_slice());
        let op = AssignmentOperator::from(self.operator.as_str());
        token_stream.extend_from_slice(self.right.to_token().as_slice());
        token_stream
    }
}

impl ToToken for MemberExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        token_stream.extend_from_slice(self.property.to_token().as_slice());
        token_stream
    }
}

impl ToToken for CallExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        let mut callee = String::new();
        if let box Expression::Identifier(i) = &self.callee {
            callee = i.name.clone();
        }

        let mut args = vec![];
        self.arguments.iter().for_each(|arg|
            args.extend_from_slice(arg.to_token().as_slice())
        );

        let call = FunctionToken(Call { args, callee });

        token_stream.push(call);
        token_stream
    }
}

impl ToToken for ObjectExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        self.properties.iter().for_each(|prop| {
            token_stream.extend_from_slice(prop.value.to_token().as_slice());
            token_stream.extend_from_slice(prop.key.to_token().as_slice());
        });
        token_stream
    }
}

pub fn postfix(tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut postfix_expression = vec![];
    let mut stack = Stack::new();

    while let Some(token_in) = tokens.pop() {
        println!("{:?}", stack);
        let op_on_stack = stack.op_queue.last();
        match &token_in {
            LiteralToken(_) => postfix_expression.push(token_in.clone()),
            FunctionToken(_) => stack.op_queue.push(token_in.clone()),
            OperatorToken(LeftParenthesis) => stack.op_queue.push(OperatorToken(LeftParenthesis)),
            OperatorToken(RightParenthesis) => {
                while !is_left_parenthesis(&token_in) {
                    let operator = stack.op_queue.pop().expect("Missing left parenthesis");
                    if !is_left_parenthesis(&operator) {
                        postfix_expression.push(operator);
                    };
                };
            }
            OperatorToken(op_in) => {
                while let Some(OperatorToken(operator_from_stack)) = stack.op_queue.last() {
                    let operator_from_stack = operator_from_stack.clone();
                    if (top_operator_is_function(&OperatorToken(operator_from_stack)) ||
                        operator_on_stack_as_greater_precedence(&OperatorToken(operator_from_stack), &OperatorToken(op_in.clone())) ||
                        operator_on_stack_as_equal_precedence_and_is_left_associative(&OperatorToken(operator_from_stack), &OperatorToken(op_in.clone()))) &&
                        top_operator_is_left_parenthesis(&OperatorToken(operator_from_stack)) {
                        let token = stack.op_queue.pop().unwrap();
                        postfix_expression.push(token);
                    };
                };
                stack.op_queue.push(OperatorToken(*op_in));
            }
            IdendifierToken(_) => unimplemented!(),
            Undefined => unimplemented!(),
        };
    }

    while !stack.op_queue.is_empty() {
        let leftover = stack.op_queue.pop();
        postfix_expression.push(leftover.unwrap());
    };
    postfix_expression
}

fn is_left_parenthesis(token: &Token) -> bool {
    if let OperatorToken(LeftParenthesis) = token {
        true
    } else {
        false
    }
}

fn operator_on_stack_as_greater_precedence(op_from_stack: &Token, op_from_queue: &Token) -> bool {
    let op_from_stack = match op_from_stack {
        OperatorToken(operator) => operator,
        _ => panic!("not an op")
    };

    let op_in = match op_from_queue {
        OperatorToken(operator) => operator,
        _ => panic!("not an op")
    };

    if Operator::get_precedence(&op_from_stack, &op_in) {
        true
    } else {
        false
    }
}

fn operator_on_stack_as_equal_precedence_and_is_left_associative(op_from_stack: &Token, op_from_queue: &Token) -> bool {
    let op_from_stack = match op_from_stack {
        OperatorToken(operator) => operator,
        _ => panic!("not an op")
    };

    let op_in = match op_from_queue {
        OperatorToken(operator) => operator,
        _ => panic!("not an op")
    };

    if !Operator::get_precedence(&op_from_stack, &op_in) && op_from_stack.is_left_associative() {
        true
    } else {
        false
    }
}

fn top_operator_is_function(top_operator: &Token) -> bool {
    if let FunctionToken(_) = top_operator {
        true
    } else {
        false
    }
}

fn top_operator_is_left_parenthesis(top_operator: &Token) -> bool {
    if let OperatorToken(LeftParenthesis) = top_operator {
        true
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::token::token::Token::LiteralToken;
    use crate::token::token::Literal::NumericLiteral;
    use crate::token::token::Token::OperatorToken;
    use crate::token::token::Operator;
    use crate::token::token::Operator::*;
    use crate::token::token::UnaryOperator::Plus;
    use crate::token::to_token::postfix;
    use crate::token::token::Token;
    use crate::token::token::BinaryOperator::*;

    #[test]
    fn should_postfix_expression() {
        let token_in: &mut Vec<Token> = &mut Vec::new();
        token_in.push(LiteralToken(NumericLiteral(1.0)));
        token_in.push(OperatorToken(Operator::BinOp(Add)));
        token_in.push(LiteralToken(NumericLiteral(1.0)));

        let mut expected = vec![];
        expected.push(LiteralToken(NumericLiteral(1.0)));
        expected.push(LiteralToken(NumericLiteral(1.0)));
        expected.push(OperatorToken(Operator::BinOp(Add)));
        let token_postfix = postfix(token_in);

        assert_eq!(token_postfix, expected);
    }

    #[test]
    fn should_postfix_parenthesized_expression() {
        let token_in: &mut Vec<Token> = &mut Vec::new();
        token_in.push(LiteralToken(NumericLiteral(2.0)));
        token_in.push(OperatorToken(Operator::BinOp(Mul)));
        token_in.push(OperatorToken(LeftParenthesis));
        token_in.push(LiteralToken(NumericLiteral(1.0)));
        token_in.push(OperatorToken(Operator::BinOp(Add)));
        token_in.push(LiteralToken(NumericLiteral(1.0)));
        token_in.push(OperatorToken(RightParenthesis));

        let mut expected = vec![];
        let token_postfix = postfix(token_in);

        assert_eq!(token_postfix, expected);
    }
}
