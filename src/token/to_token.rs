use std::collections::HashMap;

use crate::ast::expression::Expression::*;
use crate::ast::expression::*;
use crate::ast::statement::Statement;
use crate::ast::statement::Statement::*;
use crate::token::token::*;
use crate::token::token::Operator::*;
use crate::token::token::Token::*;
use crate::visitor::Visitor;

impl RootNode {
    pub fn build(ast: Vec<Box<Statement>>) -> RootNode {
        let mut ast_mut = ast.clone();
        let mut token_root = RootNode {
            functions: HashMap::new(),
            vars: HashMap::new(),
            main: vec![],
        };

        let mut indexes_to_remove = vec![];
        ast_mut.iter_mut().enumerate().for_each(|(i, statement)| {
            if let box VariableDeclaration(var) = statement {
                token_root.visit_variable_declaration(var);
                indexes_to_remove.push(i);
            };
        });

        indexes_to_remove.iter().for_each(|i| { ast_mut.remove(*i); });
        let mut indexes_to_remove = vec![];

        ast.iter().enumerate().for_each(|(i, statement)| {
            if let box FunctionDeclaration(function) = statement {
                token_root.visit_function_declaration(function);
                indexes_to_remove.push(i);
            };
        });

        indexes_to_remove.iter().for_each(|i| { ast_mut.remove(*i); });

        ast.iter().enumerate().for_each(|(i, statement)| {
            token_root.visit_statement(statement);
        });

        token_root
    }
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
            box Expression::ObjectExpression(object) => object.to_token(),
        }
    }
}

impl ToToken for BinaryExp {
    fn to_token(&self) -> Vec<Token> {
        let mut token_stream = vec![];
        if let Some(extra) = &self.extra { token_stream.push(Token::OperatorToken(LeftParenthesis)); }
        token_stream.extend_from_slice(self.left.to_token().as_slice());
        let op: BinaryOperator = BinaryOperator::from(self.operator.as_str());
        token_stream.push(Token::OperatorToken(BinOp(op)));
        token_stream.extend_from_slice(self.right.to_token().as_slice());
        if let Some(extra) = &self.extra { token_stream.push(Token::OperatorToken(RightParenthesis)); };
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