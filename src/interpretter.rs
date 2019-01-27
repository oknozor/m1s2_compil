use crate::ast::expression::{Expression, BinaryExpression, AssignmentExpression, CallExpression, Identifier,
                             LogicalExpression, UnaryExpression, MemberExpression};
use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclaration, VariableDeclarator, IfStatement, ForStatement, BreakStatement,
                            ContinueStatement, FunctionDeclaration, ReturnStatement, SwitchStatement};
use super::ast::expression::UpdateExpression;
use crate::runner::visitor::Visitor;
use crate::runner::handler::Handler;
use crate::ast::literal::Literal;
use crate::ast::literal::Number;
use crate::ast::expression::Extra;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::ast::operator::BinaryOp;
use std::option::Option;
use crate::ast::token::Token;
use std::collections::HashMap;
use std::any::Any;
use crate::runner::context::LexicalEnv;
use crate::ast::operator::BinaryOp::RightParenthesis;
use crate::ast::operator::BinaryOp::LeftParenthesis;
use std::cell::RefCell;
use crate::ast::token::Token::StandardFunction;
use crate::runner::standard_lib::Function;

pub struct Interpretter {
    pub global_scope: HashMap<String, Token>,
    pub local_scope: HashMap<String, Token>,
    pub stacks: HashMap<String, Stack>,
}

pub struct Stack {
    pub out_queue: Vec<Token>,
    pub operator_stack: Vec<BinaryOp>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            out_queue: vec![],
            operator_stack: vec![],
        }
    }
}


impl Visitor for Interpretter {}

impl Handler for Interpretter {

    fn handle_program_root(&mut self) {
        self.global_scope.insert(String::from("print"), Token::StandardFunction(Function::Print));
    }
    fn handle_variable_declarator(&mut self, var: &VariableDeclarator) {
        self.visit_option_expression(&var.init);
        let value = self.stack.out_queue.pop().unwrap_or_else(|| Token::Undefined);
        self.local_scope.insert(var.id.name.clone(), value);
        self.print_local_scope();
    }

    fn handle_expression_statement(&mut self, s: &ExpressionStatement) {
        self.stack.out_queue = vec![];
        self.stack.operator_stack = vec![];
    }

    fn handle_expression(&mut self, exp: &Expression) {

    }

    fn handle_binary_expression(&mut self, binary_exp: &BinaryExpression) {
        self.print_local_scope();
        self.visit_expression(&binary_exp.left);
        let operator = BinaryOp::from_str(&binary_exp.operator);
        self.stack.out_queue.push(Token::BinaryOp(operator));
        self.visit_expression(&binary_exp.right);
    }

    fn handle_string_literal(&mut self, literal: &Literal<String>) {
        let string_token = Token::StringLiteral(literal.value.clone());
        self.stack.out_queue.push(string_token);
    }

    fn handle_numeric_literal(&mut self, literal: &Literal<i64>) {
        self.stack.out_queue.push(Token::NumericLiteral(Number(literal.value)));
    }

    fn handle_start_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            self.stack.out_queue.push(Token::BinaryOp(LeftParenthesis));
        }
    }

    fn handle_end_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            self.stack.out_queue.push(Token::BinaryOp(RightParenthesis));
        }
    }

    fn handle_call_expression(&mut self, func: &CallExpression) {
        self.visit_expression(&func.callee);
    }

    fn on_statement_end(&mut self) {
        self.print_out_queue();
    }

    fn handle_identifier(&mut self, id: &String) {
        self.print_local_scope();
        if !self.local_scope.is_empty() {
            let local_token = self.local_scope.get(id);
            if let Some(Token) = local_token {
                let local_token = local_token.unwrap();
            } else {
                let global_token = self.global_scope.get(id);
            }
        }
    }

    fn handle_while_statement(&mut self, w: &WhileStatement) {
        self.visit_expression(&w.test);
        let test: bool = self.get_expression_as_single_token(vec![Token::BooleanLiteral(true)]);
        while (test) {
            self.visit_statement(&w.body);
            println!("currently visiting a while statement ! ")

        }
    }
}


impl Interpretter {
    /*    fn solve_expression_literal {

            //START
            let operator = NumOp::from_str(&binary_exp.operator);
            // while operator on the stack as precedence over the one being evaluated
            // and it's not a parenthesis, pop it and push it to queue
            while !self.operator_stack.is_empty()
                && (NumOp::get_precedence(self.operator_stack.last().unwrap(), &operator)
                && self.operator_stack.last().unwrap() != &NumOp::LeftParenthesis) {
                let pop_op = self.operator_stack.pop().unwrap();
                self.out_queue.push(Token::NumOp(pop_op));


            // then push the current one on the operator stack
            //PAR
            // we get a right parenthesis, get the matching operator
            // pop 2 number from the queue and calculate the operation
            // and push the result on the output queue
            while self.operator_stack.last().unwrap() != &NumOp::LeftParenthesis {
                let pop_op = self.operator_stack.pop().unwrap();
                let pop_op = Token::NumOp(pop_op);
                let a = self.out_queue.pop().unwrap();
                let b = self.out_queue.pop().unwrap();
                println!("calculating {} {} {}", b.to_string(), pop_op.to_string(), b.to_string());
                let r = Token::calc(a, b, pop_op);
                self.out_queue.push(r);
            }
            self.operator_stack.pop(); // pop left parenthesis


            //END
            while !self.operator_stack.is_empty() {
                let pop_op = self.operator_stack.pop().unwrap();
                let a = self.out_queue.pop().unwrap();
                let b = self.out_queue.pop().unwrap();
                println!("calculating {} {} {}", b.to_string(), pop_op.to_string(), a.to_string());
                let r = Token::calc(a, b, Token::NumOp(pop_op));
                self.out_queue.push(r);
            }
            let temp = &self.out_queue.pop().unwrap().clone();
            println!("Statement result: {}", temp.to_string());
        }*/

    fn push_to_queue(&mut self, token: Token) {
        self.stack.out_queue.push(token);
    }

    fn pop_from_queue(&mut self) -> Token {
        let result = self.stack.out_queue.pop();
        if let Some(token) = result {
            token
        } else {
            panic!("Expected a token, found None")
        }
    }

    fn push_to_op_stack(&mut self, token: Token) {
        let token_as_bin_op: BinaryOp = token.into();
        self.stack.operator_stack.push(token_as_bin_op);
    }

    fn pop_from_op_stack(&mut self, token: Token) -> Token {
        let result = self.stack.operator_stack.pop();
        if let Some(binop) = result {
            Token::BinaryOp(binop)
        } else {
            panic!("Expected a bin op, found None")
        }
    }

    fn get_expression_as_single_token(&mut self, expression: Vec<Token>) -> bool {
        true
    }

    fn print_out_queue(&mut self) {
        print!("Queue [ ");
        self.stack.out_queue.iter().for_each(|x| print!("{} ", x));
        print!(" ]");
        println!();
    }

    fn print_local_scope(&mut self) {
        print!("\nCurrent scope holding: [");
        self.local_scope.iter().for_each(|reference| print!("{}:{}, ", reference.0, reference.1));
        print!("]");
    }
}
